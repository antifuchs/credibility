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
        let mut tb: credibility::TestBlock<TestTracker> =
            credibility::TestBlock::new("foo", &mut tracker);
        aver!(tb, false); // This gets executed
        aver!(tb, true); // This too, despite the panic above!
    }
    assert_eq!(tracker.counts(), (1, 1, 0, 0, 0));
}

#[test]
fn err_result() {
    let mut tracker = TestTracker::default();
    {
        defer_test_result!(tb, tracker, "asserting that failure happens", {
            failure_result()
        });
    }
    assert_eq!(tracker.counts(), (0, 0, 1, 0, 0));
}

#[test]
fn ok_result() {
    let mut tracker = TestTracker::default();
    {
        defer_test_result!(tb, tracker, "asserting that success is OK", { Ok(()) });
    }
    assert_eq!(tracker.counts(), (0, 0, 0, 1, 0));
}
