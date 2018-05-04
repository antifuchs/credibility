extern crate failure;

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

pub fn guard_against_panic<St, R>(
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
        defer_test_result!($block, tracker: DefaultStatusTracker, $name, $code);
    };
}
