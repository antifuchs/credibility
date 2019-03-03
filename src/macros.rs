/// Ensure that a boolean expression is `true` and cause a test block
/// to fail if it is false.
///
/// This macro behaves the same way as `assert!` does, except it will
/// not immediately halt execution of a code block. This is most
/// useful in tests where you want to see the result of multiple
/// inputs in a given configuration (e.g. in table-driven tests.)
///
/// # Examples
/// ``` rust
/// # #[macro_use] extern crate credibility;
/// # fn main() {
/// test_block!(tb, "An example test block", {
///     aver!(tb, true, "A working assertion");
/// });
/// # }
/// ```
///
/// And an example of failing test cases; note that all the cases will
/// be checked and contribute to the panic at the end:
///
/// ``` rust,should_panic
/// # #[macro_use] extern crate credibility;
/// # fn main() {
/// test_block!(tb, "An example test block", {
///     let cases = vec![
///         (2, 3, 5),  // will be checked
///         (1, 1, 2),  // will also be checked!
///         (1, 1, 3),  // and this, too (the single successful test case)
///     ];
///     for (in1, in2, output) in cases {
///         aver!(tb, in1+in2 != output);
///     }
/// });
/// # }
/// ```
#[macro_export]
macro_rules! aver {
    ($block:ident, $($arg:tt)+) => {
        $block.eval_aver(|| {
            assert!($($arg)+);
        });
    };
}

/// Asserts that two values are equal to each other (using
/// `assert_eq`), but does not panic.
///
/// Much like [`aver!`](#macro.aver), `aver_eq!` uses `assert_eq!` and
/// catches any panic caused; if any panic occurs, it tells the test
/// block which will fail once it gets dropped.
///
/// # Examples
/// ```
/// # #[macro_use] extern crate credibility;
/// # fn main() {
/// test_block!(tb, "An example test block", {
///     aver_eq!(tb, 2+3, 5, "Math should work!");
/// });
/// # }
/// ```
///
/// And here's the example of failing test cases above again, just
/// with nicer error messages. Again, note that all the cases will be
/// checked and contribute to the panic at the end:
///
/// ``` rust,should_panic
/// # #[macro_use] extern crate credibility;
/// # fn main() {
/// test_block!(tb, "An example test block", {
///     let cases = vec![
///         (2, 3, 5),  // will be checked
///         (1, 1, 2),  // will also be checked!
///         (1, 1, 3),  // and this, too (the single successful test case)
///     ];
///     for (in1, in2, output) in cases {
///         let result = in1+in2+1;
///         aver_eq!(tb, output, result);
///     }
/// });
/// # }
/// ```
#[macro_export]
macro_rules! aver_eq {
    ($block:ident, $($arg:tt)+) => {
        $block.eval_aver(|| {
            assert_eq!($($arg)+);
        });
    };
}

/// Asserts that two values are not equal to each other (using
/// `assert_ne`), but does not panic.
///
/// Much like [`aver!`](#macro.aver), `aver_ne!` uses `assert_ne!` and
/// catches any panic caused; if any panic occurs, it tells the test
/// block which will fail once it gets dropped.
///
/// # Examples
/// ```
/// # #[macro_use] extern crate credibility;
/// # fn main() {
/// test_block!(tb, "An example test block", {
///     aver_ne!(tb, 2+4, 5, "Math should work!");
/// });
/// # }
/// ```
///
/// And here's the example of failing test cases above again, just
/// with nicer error messages. Again, note that all the cases will be
/// checked and contribute to the panic at the end:
///
/// ``` rust,should_panic
/// # #[macro_use] extern crate credibility;
/// # fn main() {
/// test_block!(tb, "An example test block", {
///     let cases = vec![
///         (2, 3, 5),  // will be checked
///         (1, 1, 2),  // will also be checked!
///         (1, 1, 3),  // and this, too (the single successful test case)
///     ];
///     for (in1, in2, output) in cases {
///         let result = in1+in2;
///         aver_ne!(tb, output, result);
///     }
/// });
/// # }
/// ```
#[macro_export]
macro_rules! aver_ne {
    ($block:ident, $($arg:tt)+) => {
        $block.eval_aver(|| {
            assert_ne!($($arg)+);
        });
    };
}

/// Create a [`TestBlock`](struct.TestBlock.html) valid for a block of code.
///
/// `test_block` is a convenience macro (that's very convenient!) for
/// running tests in bulk and causing a test abort based on their
/// results once the block terminates.
///
/// # Teardown behavior
/// The behavior at the end of the block depends on the
/// [`TestReporter`](trait.TestReporter.html) used; the default form
/// of this macro creates a
/// [`DefaultTestReporter`](struct.DefaultTestReporter.html), which
/// panics at the end of the block if any errors occur, or if the code
/// block returns a non-`Ok` result.
///
/// Use the form of this macro that takes an additional
/// [`TestReporter`](trait.TestReporter.html) argument to customize
/// this behavior; see the module [`selftest`](selftest/index.html)
/// for an example.
///
/// # Compatibility with test functions that return `Result`s
///
/// `test_block` properly treats blocks that use the `?` postfix operator
/// to handle `Result`s in tests that return `Result` types (see [the
/// book][using-results-in-tests]). The test block block is evaluated
/// inside the test function, and whenever a `?` usage would abort and
/// return an error result early, the test function will do just that.
///
/// In other words, using `?` in these functions is the equivalent of
/// [`testify`][testify]'s `require` functions, which will abort and fail the test
/// early because it can not continue.
///
/// [using-results-in-tests]: https://doc.rust-lang.org/book/ch11-01-writing-tests.html#using-resultt-e-in-tests
/// [testify]: https://github.com/stretchr/testify#require-package
///
/// # Examples
///
/// A functioning example of a table test:
/// ``` rust
/// # #[macro_use] extern crate credibility;
/// # fn main() {
/// test_block!(tb, "An example test block", {
///     let cases = vec![
///         (2, 3, 5),
///         (1, 1, 2),
///         (1, 1, 2),
///     ];
///     for (in1, in2, output) in cases {
///         aver_eq!(tb, output, in1+in2);
///     }
/// });
/// # }
/// ```
///
/// An example of how to handle error results in tests:
/// ``` rust,should_panic
/// # #[macro_use] extern crate credibility;
/// # fn fail() -> Result<(), &'static str> { Err("this test should fail") }
/// # fn main() -> Result<(), &'static str> {
/// test_block!(tb, "An example test block that should fail", {
///     fail()?;
///     Ok(())
/// })
/// # }
/// ```
#[macro_export]
macro_rules! test_block {
    ($block:ident, $tracker:ident, $name:expr, $code:block) => {{
        let mut $block = $crate::TestBlock::new($name, &mut $tracker);
        let res = { $code };
        // $code might panic, so this might never be reached:
        #[allow(unreachable_code)]
        {
            $block.finished();
            res
        }
    }};
    ($block:ident, $name:expr, $code:block) => {{
        let mut tracker = $crate::DefaultTestReporter::default();
        test_block!($block, tracker, $name, $code)
    }};
}
