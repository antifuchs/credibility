extern crate failure;
#[macro_use]
extern crate failure_derive;

use std::fmt::Display;
use std::panic::{catch_unwind, AssertUnwindSafe};

pub trait StatusTracker {
    fn failed(&mut self, expectation: impl Expectation);
    fn succeeded(&mut self, expectation: impl Expectation);
    fn tally<'a>(&self, name: &'a str);
}

pub struct TestBlock<'a, St>
where
    St: 'a + StatusTracker + Sized,
{
    name: &'a str,
    status_tracker: Option<&'a mut St>,
}

impl<'a, St> TestBlock<'a, St>
where
    St: StatusTracker + Sized,
{
    pub fn new<S: Into<Option<&'a mut St>>>(name: &'a str, tracker: S) -> TestBlock<'a, St> {
        TestBlock {
            name: name,
            status_tracker: tracker.into(),
        }
    }
}

impl<'a, St> Drop for TestBlock<'a, St>
where
    St: StatusTracker + Sized,
{
    fn drop(&mut self) {
        if let Some(ref tracker) = self.status_tracker {
            tracker.tally(self.name);
        }
    }
}

pub trait Expectation
where
    Self: Display + Sized,
{
    fn expect(self) -> Result<(), Error<Self>>;
}

impl Expectation for bool {
    fn expect(self) -> Result<(), Error<Self>> {
        if self {
            Ok(())
        } else {
            Err(Error { ex: self })
        }
    }
}

pub struct DefaultStatusTracker {
    failed: bool,
}

impl StatusTracker for DefaultStatusTracker {
    fn failed(&mut self, _expectation: impl Expectation) {
        self.failed = true
    }

    fn succeeded(&mut self, _expectation: impl Expectation) {}

    fn tally<'a>(&self, name: &'a str) {
        if self.failed {
            panic!("Test cases in block {} failed", name);
        }
    }
}

pub fn guard_against_panic<St>(
    block: &mut TestBlock<St>,
    ex: impl Expectation,
    closure: impl FnOnce(),
) where
    St: StatusTracker + Sized,
{
    let res = catch_unwind(AssertUnwindSafe(closure));
    match &mut block.status_tracker {
        &mut Some(ref mut st) => match res {
            Ok(()) => st.succeeded(ex),
            Err(_) => st.failed(ex),
        },
        None => {}
    };
}

#[derive(Fail, Debug)]
#[fail(display = "Failed expectation: {}", ex)]
pub struct Error<Ex: Expectation + Sized + Display> {
    ex: Ex,
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
        fn failed(&mut self, _expectation: impl Expectation) {
            self.failed += 1;
        }

        fn succeeded(&mut self, _expectation: impl Expectation) {
            self.succeeded += 1;
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
            let ex = true; // TODO: should make something else
            guard_against_panic(&mut tb, ex, || {
                assert!(false);
            });
        }
        assert_eq!(tracker.succeeded, 0);
        assert_eq!(tracker.failed, 1);
    }
}
