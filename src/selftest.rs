///! Structs and functions to enable testing `credibility` itself
use StatusTracker;

use std::fmt::Debug;
use std::thread;

use failure;

/// A test reporter that counts the number of things that
/// happened. It's mostly useful for writing tests.
#[derive(Copy, Clone)]
pub struct TestTracker {
    pub failed: usize,
    pub errored: usize,
    pub succeeded: usize,
    pub ran: usize,
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
    fn tally<'a>(&self, _name: &'a str) {}
}
