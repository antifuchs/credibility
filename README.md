# credibility - Macros&Types for making non-panicking assertions

This crate helps you write table-driven tests in Rust, the way you
might have been writing them in Go.

(I hope you're still here.)

This crate offers a macro, `test_block!` that allows you to defer test
failures until all assertions inside the test block have had a chance
to execute. Also, `test_block!` also lets you handle `Err` results
with `?`, so that you can more easily short-circuit out of setup code
without having to resort to calling `.unwrap()` on `Result` return
types too much.

## Examples

``` rust
#[macro_use] extern crate credibility;

#[test]
fn test_sums() {
    test_block!(tb, "An example test block", {
        let cases = vec![
            (2, 3, 5),
            (1, 1, 2),
            (1, 1, 2),
        ];
        for (in1, in2, output) in cases {
            aver_eq!(tb, output, in1+in2+1);
        }
        Ok(())
    });
}
```

This test will check all three examples given, even though they all
fail. The output looks like this:

```
     Running target/debug/deps/example-d766d73311bcb7d0

running 1 test
test test_sums ... FAILED

failures:

---- test_sums stdout ----
        thread 'test_sums' panicked at 'assertion failed: `(left == right)`
  left: `5`,
 right: `6`', tests/example.rs:10:13
note: Run with `RUST_BACKTRACE=1` for a backtrace.
thread 'test_sums' panicked at 'assertion failed: `(left == right)`
  left: `2`,
 right: `3`', tests/example.rs:10:13
thread 'test_sums' panicked at 'assertion failed: `(left == right)`
  left: `2`,
 right: `3`', tests/example.rs:10:13
thread 'test_sums' panicked at 'Test cases in block "An example test block" failed', src/reporter.rs:54:13


failures:
    test_sums

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

Much better than writing tons of test functions (and you can re-use
setup code, too)!

## Background

One of my favorite features in go is the ability to have tests that
fail but still execute all their code; this allows you to test
multiple examples at once, and see all the examples that the current
code does not handle correctly. In contrast to Rust's default test
assertion mechanism, this is much more ergonomic: Seeing multiple
failing examples allows you to narrow down the root cause for a bug
much quicker than fixing them one-by-one. Here's a simple go example
to illustrate what I mean:

``` go
func TestSums(t *testing.T) {
    tests := []struct {
        in1, in2 int
        out      int
    }{
        {1, 1, 2},
        {1, 2, 3},
        {4, 5, 6},
    }
    for _, elt := range tests {
        test := elt
        t.Run(fmt.Sprintf("Sum %d+%d=%d", test.in1, test.in2, test.out), func(t *testing.T) {
            t.Parallel()
            sum := test.in1 + test.in2
            if sum != test.out {
                t.Errorf("Sum is wrong. Expected %d, got %d", test.out, sum)
            }
        })
    }
}
```

The output for that would be:
```
$ go test -test.v -run=TestSums .
=== RUN   TestSums
=== RUN   TestSums/Sum_1+1=2
=== PAUSE TestSums/Sum_1+1=2
=== RUN   TestSums/Sum_1+2=3
=== PAUSE TestSums/Sum_1+2=3
=== RUN   TestSums/Sum_4+5=6
=== PAUSE TestSums/Sum_4+5=6
=== CONT  TestSums/Sum_1+1=2
=== CONT  TestSums/Sum_4+5=6
=== CONT  TestSums/Sum_1+2=3
--- FAIL: TestSums (0.00s)
    --- PASS: TestSums/Sum_1+1=2 (0.00s)
    --- FAIL: TestSums/Sum_4+5=6 (0.00s)
        sums_test.go:363: Sum is wrong. Expected 6, got 9
    --- PASS: TestSums/Sum_1+2=3 (0.00s)
FAIL
FAIL    github.com/antifuchs/example     0.023s
```
