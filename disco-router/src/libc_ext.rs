use std::io::Error;

pub const ETH_P_ALL: i32 = 0x0003;
pub const SIOCGIFINDEX: u64 = 0x8933;

/// Assert some predicate or panic (with a friendly error message)
///
/// __Parameters__
///   - `pred`     - Predicate to evaluate
///   - `activity` - Name of activity for prettier panic messages
///
pub fn assert<F>(pred: F, activity: &str)
where
    F: Fn() -> bool,
{
    if pred() {
        return;
    } // everything is fine

    if let Some(errno) = Error::last_os_error().raw_os_error() {
        panic!("Assertion failed for '{}'. (errno: {})", activity, errno);
    } else {
        panic!("Assertion failed for '{}'... sorry!", activity);
    }
}
