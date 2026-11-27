//! The host's deferred writes, end to end: a recording let go without
//! running lets go the query slots it resolved (a test of `Deferred`'s drop).

use super::Deferred;
use crate::queue::Written;
use std::sync::atomic::Ordering;

/// A recording let go without running (the title resets or releases a
/// closed list it never executed) never writes its resolved query
/// results: their read-back buffers are let go with it instead of
/// staying in the results map, one per ResolveQueryData, for the run's
/// life. Values and timestamps have nothing to let go.
#[test]
fn a_recording_let_go_unrun_lets_go_its_query_slots() {
    let before = crate::query::FORGOTTEN.load(Ordering::Relaxed);
    let mut deferred = Deferred::default();
    deferred.writes.push((0x1000, Written::Query(11), 8));
    deferred.writes.push((0x1008, Written::Value(1), 8));
    deferred.writes.push((0x1010, Written::SystemTimestamp, 8));
    drop(deferred);
    assert_eq!(crate::query::FORGOTTEN.load(Ordering::Relaxed), before + 1);
}
