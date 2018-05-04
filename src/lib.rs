#[macro_use]
extern crate failure;
#[macro_use]
extern crate failure_derive;

use std::fmt::Debug;
use std::panic::{catch_unwind, UnwindSafe};
use std::thread;

pub trait StatusTracker {
    fn averred<T: Sized + Debug>(&mut self, result: thread::Result<Result<T, failure::Error>>);
    fn ran<T: Sized + Debug>(&mut self, result: Result<T, failure::Error>);
    fn tally<'a>(&self, name: &'a str);
}

pub struct TestBlock<'a, St>
where
    St: 'a + StatusTracker + Sized,
{
    name: &'a str,
    status_tracker: &'a mut St,
}

impl<'a, St> TestBlock<'a, St>
where
    St: StatusTracker + Sized,
{
    pub fn new(name: &'a str, tracker: &'a mut St) -> TestBlock<'a, St> {
        TestBlock {
            name: name,
            status_tracker: tracker,
        }
    }

    pub fn run<F: Sized + Fn() -> Result<(), failure::Error>>(&mut self, fun: F) {
        self.status_tracker.ran(fun());
    }
}

impl<'a, St> Drop for TestBlock<'a, St>
where
    St: StatusTracker + Sized,
{
    fn drop(&mut self) {
        self.status_tracker.tally(self.name);
    }
}

pub struct DefaultStatusTracker {
    failed: bool,
    errored: bool,
}

impl Default for DefaultStatusTracker {
    fn default() -> DefaultStatusTracker {
        DefaultStatusTracker {
            failed: false,
            errored: false,
        }
    }
}

impl StatusTracker for DefaultStatusTracker {
    fn averred<T: Sized + Debug>(&mut self, result: thread::Result<Result<T, failure::Error>>) {
        match result {
            Err(_) => self.failed = true,
            Ok(Err(_)) => self.errored = true,
            Ok(Ok(_)) => {}
        };
    }

    fn ran<T: Sized + Debug>(&mut self, result: Result<T, failure::Error>) {
        match result {
            Err(_) => self.errored = true,
            Ok(_) => {}
        }
    }

    fn tally<'a>(&self, name: &'a str) {
        if self.failed {
            panic!("Test cases in block {:?} failed", name);
        }
    }
}

#[allow(dead_code)] // TODO: is this right? Extract tests into a crate and find out.
pub(crate) fn guard_against_panic<St, R>(
    block: &mut TestBlock<St>,
    closure: impl FnOnce() -> Result<R, failure::Error> + UnwindSafe,
) where
    St: StatusTracker + Sized,
    R: Debug + Sized,
{
    let res = catch_unwind(closure);
    block.status_tracker.averred(res);
}

#[macro_export]
macro_rules! aver {
    ($block:expr, $statement:expr) => {
        guard_against_panic(&mut $block, || -> Result<(), failure::Error> {
            assert!($statement);
            Ok(())
        });
    };
    ($block:expr, $statement:expr, $($arg:tt)+) => {
        guard_against_panic(&mut $block, || -> Result<(), failure::Error> {
            assert!($statement, $($arg)+);
            Ok(())
        });
    };
}

#[macro_export]
macro_rules! defer_test_result {
    ($block:ident, $tracker:ident : $tracker_type:ty, $name:expr, $code:block) => {
        let mut $block: TestBlock<$tracker_type> = TestBlock::new($name, &mut $tracker);
        let fun = || -> Result<(), failure::Error> {
            $code
        }
        $block.run(fun);
    };
    ($block:ident, $name:expr, $code:block) => {
        let mut tracker = DefaultStatusTracker::default();
        let mut $block: TestBlock<DefaultStatusTracker> = TestBlock::new($name, &mut tracker);
        let fun = || -> Result<(), failure::Error> {
            $code
        }
        $block.run(fun);
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Copy, Clone)]
    struct TestTracker {
        failed: usize,
        errored: usize,
        succeeded: usize,
        ran: usize,
    }

    impl Default for TestTracker {
        fn default() -> TestTracker {
            TestTracker {
                failed: 0,
                succeeded: 0,
                errored: 0,
                ran: 0,
            }
        }
    }

    impl StatusTracker for TestTracker {
        fn averred<T: Sized + Debug>(&mut self, result: thread::Result<Result<T, failure::Error>>) {
            println!("aver result: {:?}", result);
            match result {
                Ok(Ok(_)) => self.succeeded += 1,
                Ok(Err(_)) => self.errored += 1,
                Err(_) => self.failed += 1,
            }
        }

        fn ran<T: Sized + Debug>(&mut self, result: Result<T, failure::Error>) {
            println!("run result: {:?}", result);
            match result {
                Err(_) => self.errored += 1,
                Ok(_) => self.ran += 1,
            }
        }
        fn tally<'a>(&self, _name: &'a str) {}
    }

    fn failure_result() -> Result<(), failure::Error> {
        Err(format_err!("nope!"))?;
        Ok(())
    }

    #[test]
    fn panicking() {
        let mut tracker = TestTracker::default();
        {
            let mut tb: TestBlock<TestTracker> = TestBlock::new("foo", &mut tracker);
            aver!(tb, false); // This gets executed
            aver!(tb, true); // This too, despite the panic above!
        }
        assert_eq!(
            (
                tracker.failed,
                tracker.succeeded,
                tracker.errored,
                tracker.ran
            ),
            (1, 1, 0, 0)
        );
    }

    #[test]
    fn err_result() {
        let mut tracker = TestTracker::default();
        {
            defer_test_result!(
                tb,
                tracker: TestTracker,
                "asserting that failure happens",
                { failure_result() }
            );
        }
        assert_eq!(
            (
                tracker.failed,
                tracker.succeeded,
                tracker.errored,
                tracker.ran
            ),
            (0, 0, 1, 0)
        );
    }

    #[test]
    fn ok_result() {
        let mut tracker = TestTracker::default();
        {
            defer_test_result!(tb, tracker: TestTracker, "asserting that success is OK", {
                Ok(())
            });
        }
        assert_eq!(
            (
                tracker.failed,
                tracker.succeeded,
                tracker.errored,
                tracker.ran
            ),
            (0, 0, 0, 1)
        );
    }
}
