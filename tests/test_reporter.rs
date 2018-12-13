// #![feature(trace_macros)]

#[macro_use]
extern crate credibility;

use std::panic::catch_unwind;

#[test]
fn panic_with_default_reporter() {
    match catch_unwind(|| {
        test_block!(_inner_tb, "Block with a default test reporter", {
            panic!("hey hey");
        });
    }) {
        Ok(_) => panic!("Should have properly panicked"),
        Err(_) => {} // Ok.
    }
}

#[test]
fn aver_with_default_reporter() {
    assert!(catch_unwind(|| {
        test_block!(inner_tb, "Block with a default test reporter", {
            aver!(inner_tb, false, "Executed");
            aver!(inner_tb, false, "Also executed");
        });
    })
    .is_err());
}

fn error_result() -> Result<(), &'static str> {
    return Err("I should fail");
}

#[test]
fn err_result_with_default_reporter() {
    assert!(catch_unwind(|| {
        test_block!(inner_tb, "Block with a default test reporter", {
            error_result()
        });
    })
    .is_err());
}
