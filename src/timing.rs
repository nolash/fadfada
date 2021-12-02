/// [Scheduler] is used with [Controller](crate::control::Controller) and [Source](crate::source::Source) to define query offsets and timeouts on a
/// per-session and per-source basis.
pub struct Scheduler {
    pub delay: u32,
    pub timeout: u32,
}
