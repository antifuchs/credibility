// #![feature(trace_macros)]

#[macro_use]
extern crate credibility;
#[macro_use]
extern crate failure;

use std::panic::catch_unwind;

#[test]
fn panic_with_default_reporter() {
    match catch_unwind(|| {
        defer_test_result!(_inner_tb, "Block with a default test reporter", {
            panic!("hey hey");
        });
    }) {
        Ok(_) => panic!("Should have properly panicked"),
        Err(_) => {} // Ok.
    }
}

#[test]
fn aver_with_default_reporter() {
    assert!(
        catch_unwind(|| {
            defer_test_result!(inner_tb, "Block with a default test reporter", {
                aver!(inner_tb, false, "Executed");
                aver!(inner_tb, false, "Also executed");
                Ok(())
            });
        }).is_err()
    );
}

#[test]
fn err_result_with_default_reporter() {
    assert!(
        catch_unwind(|| {
            defer_test_result!(inner_tb, "Block with a default test reporter", {
                Err(format_err!("I should fail!"))
            });
        }).is_err()
    );
}
