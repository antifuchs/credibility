use std::fmt::Debug;
use std::thread;

/// Collects and report test statuses.  This trait is used by the test
/// block type to retain the state of a test run.
pub trait TestReporter {
    /// Invoked whenever the result of an [`aver!`](#macro.aver) is
    /// available. If that result is the `thread::Result`'s `Err`
    /// kind, the `aver!` failed, indicating that the test should fail.
    fn averred<T: Sized + Debug>(&mut self, result: thread::Result<T>);

    /// Invoked whenever a test block finished. If result is an `Err`,
    /// indicates that the block failed. This means that the test
    /// should abort.
    fn ran<T: Send + Sized + Debug, E: Send + Sized + Debug>(
        &mut self,
        result: TestBlockResult<T, E>,
    );

    /// Invoked at the end of life of a test block.
    ///
    /// # Panics
    /// This method should be expected to panic if there are any test
    /// failures.
    fn tally<'a>(&self, name: &'a str);
}

/// The default test status reporter. It delays all panics from `aver`
/// and `aver_eq` invocations to the end of the test block's lifetime;
/// if any failures occurred, or if the test block returned with an
/// `Err` Result, it panics.
pub struct DefaultTestReporter {
    failed: bool,
}

impl Default for DefaultTestReporter {
    fn default() -> DefaultTestReporter {
        DefaultTestReporter { failed: false }
    }
}

impl TestReporter for DefaultTestReporter {
    fn averred<T: Sized + Debug>(&mut self, result: thread::Result<T>) {
        if result.is_err() {
            self.failed = true
        }
    }

    fn ran<T: Sized + Debug, E: Sized + Debug>(&mut self, result: TestBlockResult<T, E>) {
        if let Some(res) = result.res {
            res.expect("Unexpected error result");
        }
    }

    fn tally<'a>(&self, name: &'a str) {
        if self.failed {
            panic!("Test cases in block {:?} failed", name);
        }
    }
}

#[derive(Debug)]
pub struct TestBlockResult<T, E>
where
    T: Debug + Sized,
    E: Debug + Sized,
{
    pub res: Option<Result<T, E>>,
}

impl From<()> for TestBlockResult<(), ()> {
    fn from(_nothing: ()) -> TestBlockResult<(), ()> {
        TestBlockResult { res: None }
    }
}

impl<T: Debug + Send, E: Debug + Send> From<Result<T, E>> for TestBlockResult<T, E> {
    fn from(res: Result<T, E>) -> TestBlockResult<T, E> {
        TestBlockResult { res: Some(res) }
    }
}
