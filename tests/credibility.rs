#[macro_use]
extern crate credibility;

use credibility::selftest::*;

fn failure_result() -> Result<(), &'static str> {
    Err("nope!")
}

#[test]
fn aver_failures() {
    let mut tracker = TestTracker::default();
    {
        test_block!(
            tb,
            tracker,
            "Checking that aver failures don't cause aborts",
            {
                aver!(tb, false, "Executed");
                aver!(tb, false, "Also executed");
            }
        );
    }
    assert_eq!(tracker.counts(), (2, 0, 0, 1));
}

#[test]
fn aver_eq() {
    let mut tracker = TestTracker::default();
    {
        test_block!(
            tb,
            tracker,
            "Checking that aver failures don't cause aborts",
            {
                aver_eq!(tb, false, false, "Equal");
                aver_eq!(tb, true, false, "Not equal");
                aver_eq!(tb, true, false, "Not equal, again");
            }
        );
    }
    assert_eq!(tracker.counts(), (2, 1, 0, 1));
}

#[test]
fn aver_ne() {
    let mut tracker = TestTracker::default();
    {
        test_block!(
            tb,
            tracker,
            "Checking that aver failures don't cause aborts",
            {
                aver_ne!(tb, false, false, "Equal");
                aver_ne!(tb, true, false, "Not equal");
                aver_ne!(tb, true, false, "Not equal, again");
            }
        );
    }
    assert_eq!(tracker.counts(), (1, 2, 0, 1));
}

#[test]
fn aver_table() {
    let mut tracker = TestTracker::default();
    {
        let cases = vec![(1, 1, 2), (3, 4, 5), (5, 6, 11)];
        test_block!(
            tb,
            tracker,
            "Checking that aver failures don't cause aborts",
            {
                for (in1, in2, output) in cases {
                    let sum = in1 + in2;
                    aver_eq!(tb, sum, output);
                }
            }
        );
    }
    assert_eq!(tracker.counts(), (1, 2, 0, 1));
}

#[test]
fn aver_success() {
    let mut tracker = TestTracker::default();
    {
        test_block!(tb, tracker, "Checking that aver successes count", {
            aver!(tb, true, "Executed");
            aver!(tb, true, "Also executed");
        });
    }
    assert_eq!(tracker.counts(), (0, 2, 0, 1));
}

#[test]
fn err_result() {
    let mut tracker = TestTracker::default();
    {
        test_block!(tb, tracker, "asserting that failure happens", {
            failure_result()
        });
    }
    assert_eq!(tracker.counts(), (0, 0, 1, 0));
}

#[test]
fn ok_result() {
    let mut tracker = TestTracker::default();
    {
        test_block!(tb, tracker, "asserting that success is OK", {});
    }
    assert_eq!(tracker.counts(), (0, 0, 0, 1));
}
