//! Structs and functions to enable testing `credibility` itself
use TestReporter;

use std::fmt::Debug;
use std::thread;

use failure;

/// A test reporter that counts the number of things that
/// happened. It's mostly useful for writing tests.
#[derive(Copy, Clone)]
pub struct TestTracker {
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

/// Implements the `TestReporter` trait non-fatally. This
/// implementation does not panic, making it very useful for writing
/// tests.
impl TestReporter for TestTracker {
    fn averred<T: Sized + Debug>(&mut self, result: thread::Result<T>) {
        println!("aver result: {:?}", result);
        match result {
            Ok(_) => self.succeeded += 1,
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

    /// Does nothing. To get information about a test block's statuses
    /// in a real test, use [`counts`](#method.counts).
    fn tally<'a>(&self, _name: &'a str) {}
}

impl TestTracker {
    /// Returns a tuple containing the number of:
    /// * failed assertions
    /// * succeeded assertions
    /// * blocks that returned an Err result
    /// * blocks that returned an Ok result
    pub fn counts(&self) -> (usize, usize, usize, usize) {
        (self.failed, self.succeeded, self.errored, self.ran)
    }
}
