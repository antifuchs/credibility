extern crate failure;

use std::panic::{catch_unwind, UnwindSafe};
use std::thread;

pub trait StatusTracker {
    fn averred<T: Sized>(&mut self, result: thread::Result<T>);
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
}

impl StatusTracker for DefaultStatusTracker {
    fn averred<T: Sized>(&mut self, result: thread::Result<T>) {
        if let Err(_) = result {
            self.failed = true;
        }
    }

    fn tally<'a>(&self, name: &'a str) {
        if self.failed {
            panic!("Test cases in block {} failed", name);
        }
    }
}

pub fn guard_against_panic<St>(block: &mut TestBlock<St>, closure: impl FnOnce() + UnwindSafe)
where
    St: StatusTracker + Sized,
{
    let res = catch_unwind(closure);
    block.status_tracker.averred(res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Copy, Clone)]
    struct TestTracker {
        failed: usize,
        succeeded: usize,
    }

    impl StatusTracker for TestTracker {
        fn averred<T: Sized>(&mut self, result: thread::Result<T>) {
            match result {
                Ok(_) => self.succeeded += 1,
                Err(_) => self.failed += 1,
            }
        }

        fn tally<'a>(&self, _name: &'a str) {
            // TODO: maybe extend the interface to do asserts?
        }
    }

    #[test]
    fn panicking() {
        let mut tracker = TestTracker {
            failed: 0,
            succeeded: 0,
        };
        {
            let mut tb: TestBlock<TestTracker> = TestBlock::new("foo", &mut tracker);
            guard_against_panic(&mut tb, || {
                assert!(false);
            });
        }
        assert_eq!(tracker.succeeded, 0);
        assert_eq!(tracker.failed, 1);
    }
}
