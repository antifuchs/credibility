#[macro_use]
extern crate credibility;
#[macro_use]
extern crate failure;

use credibility::selftest::*;

fn failure_result() -> Result<(), failure::Error> {
    Err(format_err!("nope!"))?;
    Ok(())
}

#[test]
fn panicking() {
    let mut tracker = TestTracker::default();
    {
        defer_test_result!(inner_tb, tracker, "Block with a default test reporter", {
            aver!(inner_tb, false, "Executed");
            aver!(inner_tb, false, "Also executed");
            Ok(())
        });
    }
    assert_eq!(tracker.counts(), (2, 0, 0, 1));
}

#[test]
fn err_result() {
    let mut tracker = TestTracker::default();
    {
        defer_test_result!(tb, tracker, "asserting that failure happens", {
            failure_result()
        });
    }
    assert_eq!(tracker.counts(), (0, 0, 1, 0));
}

#[test]
fn ok_result() {
    let mut tracker = TestTracker::default();
    {
        defer_test_result!(tb, tracker, "asserting that success is OK", { Ok(()) });
    }
    assert_eq!(tracker.counts(), (0, 0, 0, 1));
}
