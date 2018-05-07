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
fn aver_failures() {
    let mut tracker = TestTracker::default();
    {
        defer_test_result!(
            tb,
            tracker,
            "Checking that aver failures don't cause aborts",
            {
                aver!(tb, false, "Executed");
                aver!(tb, false, "Also executed");
                Ok(())
            }
        );
    }
    assert_eq!(tracker.counts(), (2, 0, 0, 1));
}

#[test]
fn aver_success() {
    let mut tracker = TestTracker::default();
    {
        defer_test_result!(tb, tracker, "Checking that aver successes count", {
            aver!(tb, true, "Executed");
            aver!(tb, true, "Also executed");
            Ok(())
        });
    }
    assert_eq!(tracker.counts(), (0, 2, 0, 1));
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
