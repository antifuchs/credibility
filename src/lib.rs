extern crate failure;

pub mod selftest;

use std::fmt::Debug;
use std::panic::{catch_unwind, UnwindSafe};
use std::thread;

pub trait StatusTracker {
    fn panicked(&mut self);
    fn ran<T: Sized + Debug>(&mut self, result: Result<T, failure::Error>);
    fn averred<T: Sized + Debug>(&mut self, result: thread::Result<T>) {
        match result {
            Ok(_) => {}
            Err(_) => self.panicked(),
        }
    }
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

    pub fn run<F: Sized + UnwindSafe + FnOnce() -> Result<(), failure::Error>>(&mut self, fun: F) {
        let thread_res = catch_unwind(|| fun());
        match thread_res {
            Ok(result) => self.status_tracker.ran(result),
            Err(_) => self.status_tracker.panicked(),
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

impl Default for DefaultStatusTracker {
    fn default() -> DefaultStatusTracker {
        DefaultStatusTracker { failed: false }
    }
}

impl StatusTracker for DefaultStatusTracker {
    fn panicked(&mut self) {
        self.failed = true;
    }

    fn ran<T: Sized + Debug>(&mut self, result: Result<T, failure::Error>) {
        result.expect("Test block {:?} returned Err result");
    }

    fn tally<'a>(&self, name: &'a str) {
        if self.failed {
            panic!("Test block {:?} panicked", name);
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

#[macro_export]
macro_rules! aver {
    ($block:expr, $statement:expr) => {
        $crate::guard_against_panic(&mut $block, || {
            assert!($statement);
        });
    };
    ($block:expr, $statement:expr, $($arg:tt)+) => {
        $crate::guard_against_panic(&mut $block, || {
            assert!($statement, $($arg)+);
        });
    };
}

#[macro_export]
macro_rules! defer_test_result {
    ($block:ident, $tracker:ident, $name:expr, $code:block) => {{
        let mut $block = $crate::TestBlock::new($name, &mut $tracker);
        let fun = || -> Result<(), ::failure::Error> { $code };
        $block.run(fun);
    }};
    ($block:ident, $name:expr, $code:block) => {
        let mut tracker = $crate::DefaultStatusTracker::default();
        defer_test_result!($block, tracker, $name, $code);
    };
}
