extern crate failure;

pub mod selftest;

use std::fmt::Debug;
use std::panic::{catch_unwind, UnwindSafe};
use std::thread;

pub trait StatusTracker {
    fn averred<T: Sized + Debug>(&mut self, result: thread::Result<T>);
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

    pub fn ran(&mut self, res: Result<(), failure::Error>) {
        self.status_tracker.ran(res);
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

impl Default for DefaultStatusTracker {
    fn default() -> DefaultStatusTracker {
        DefaultStatusTracker { failed: false }
    }
}

impl StatusTracker for DefaultStatusTracker {
    fn averred<T: Sized + Debug>(&mut self, result: thread::Result<T>) {
        match result {
            Err(_) => self.failed = true,
            Ok(_) => {}
        };
    }

    fn ran<T: Sized + Debug>(&mut self, result: Result<T, failure::Error>) {
        result.expect("Unexpected error result");
    }

    fn tally<'a>(&self, name: &'a str) {
        if self.failed {
            panic!("Test cases in block {:?} failed", name);
        }
    }
}

pub fn aver_with<St>(block: &mut TestBlock<St>, closure: impl FnOnce() + UnwindSafe)
where
    St: StatusTracker + Sized,
{
    let res = catch_unwind(closure);
    block.status_tracker.averred(res);
}

#[macro_export]
macro_rules! aver {
    ($block:expr, $statement:expr) => {
        $crate::aver_with(&mut $block, || {
            assert!($statement);
        });
    };
    ($block:expr, $statement:expr, $($arg:tt)+) => {
        $crate::aver_with(&mut $block, || {
            assert!($statement, $($arg)+);
        });
    };
}

#[macro_export]
macro_rules! defer_test_result {
    ($block:ident, $tracker:ident, $name:expr, $code:block) => {{
        let mut $block = $crate::TestBlock::new($name, &mut $tracker);
        let result = {
            let mut fun = || -> Result<(), ::failure::Error> { $code };
            fun()
        };
        $block.ran(result);
    }};
    ($block:ident, $name:expr, $code:block) => {{
        let mut tracker = $crate::DefaultStatusTracker::default();
        defer_test_result!($block, tracker, $name, $code);
    }};
}
