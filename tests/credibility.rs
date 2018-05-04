#[macro_use]
extern crate credibility;
#[macro_use]
extern crate failure;

use credibility::*;
use std::fmt::Debug;
use std::thread;

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

impl credibility::StatusTracker for TestTracker {
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
        let mut tb: credibility::TestBlock<TestTracker> =
            credibility::TestBlock::new("foo", &mut tracker);
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
