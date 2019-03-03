use std::panic::{catch_unwind, UnwindSafe};

use super::TestReporter;

/// A RAII test result accumulator.  The `TestBlock` defines a unit of
/// test functionality, within which assertions using the macro
/// [`aver!`](macro.aver.html) can fail without aborting the test.
///
/// # Lifetime & Usage
/// As soon as a `TestBlock` variable gets dropped, it kicks off its
/// status tracker's [`tally`](trait.TestReporter.html#tymethod.tally)
/// method, which (by default) asserts that no failures occurred.
///
/// # Using results in tests
/// Since error results are the other "big" cause of early test
/// aborts, test blocks optionally allow code blocks that use them to
/// return results early with `?` or `try!`. To do so ergonomically,
/// use the [`test_block!`](macro.test_block.html) macro.
pub struct TestBlock<'a, St>
where
    St: 'a + TestReporter + Sized,
{
    name: &'a str,
    status_tracker: &'a mut St,
}

impl<'a, St> TestBlock<'a, St>
where
    St: TestReporter + Sized,
{
    /// Creates a new `TestBlock` with a tracker.
    pub fn new(name: &'a str, tracker: &'a mut St) -> TestBlock<'a, St> {
        TestBlock {
            name,
            status_tracker: tracker,
        }
    }

    /// Evaluate an `aver` expression as an `assert`, catching any
    /// panics.  Calls the test reporter's
    /// [`averred`](trait.TestReporter.html#tymethod.averred) method with
    /// the result of capturing any unwinds.
    pub fn eval_aver(&mut self, closure: impl FnOnce() + UnwindSafe) {
        let res = catch_unwind(closure);
        self.status_tracker.averred(res);
    }

    pub fn finished(&mut self) {
        self.status_tracker.ran()
    }
}

/// When dropped, `TestBlock`s call their test reporter's
/// [`tally`](trait.TestReporter.html#tymethod.tally) method.
impl<'a, St> Drop for TestBlock<'a, St>
where
    St: TestReporter + Sized,
{
    fn drop(&mut self) {
        self.status_tracker.tally(self.name);
    }
}
