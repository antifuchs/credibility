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
/// # extern crate failure;
/// # fn main() {
/// defer_test_result!(tb, "An example test block", {
///     aver!(tb, true, "A working assertion");
///     Ok(())
/// });
/// # }
/// ```
///
/// And an example of failing test cases; note that all the cases will
/// be checked and contribute to the panic at the end:
///
/// ``` rust,should_panic
/// # #[macro_use] extern crate credibility;
/// # extern crate failure;
/// # fn main() {
/// defer_test_result!(tb, "An example test block", {
///     let cases = vec![
///         (2, 3, 5),  // will be checked
///         (1, 1, 2),  // will also be checked!
///         (1, 1, 3),  // and this, too (the single successful test case)
///     ];
///     for (in1, in2, output) in cases {
///         aver!(tb, in1+in2 != output);
///     }
///     Ok(())
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
/// # extern crate failure;
/// # fn main() {
/// defer_test_result!(tb, "An example test block", {
///     aver_eq!(tb, 2+3, 5, "Math should work!");
///     Ok(())
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
/// # extern crate failure;
/// # fn main() {
/// defer_test_result!(tb, "An example test block", {
///     let cases = vec![
///         (2, 3, 5),  // will be checked
///         (1, 1, 2),  // will also be checked!
///         (1, 1, 3),  // and this, too (the single successful test case)
///     ];
///     for (in1, in2, output) in cases {
///         let result = in1+in2+1;
///         aver_eq!(tb, output, result,);
///     }
///     Ok(())
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
