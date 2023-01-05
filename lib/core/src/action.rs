//! Keep actions non-clonable.
//!
//! ## Naming
//!
//! Make sure "actions" contain enough info to be executed. A type that does not
//! contain sufficient info should not be called an action (e.g. [`TimerOp`]).

#[derive(PartialEq, Eq, Debug)]
pub enum Action {
    TimerAction { id: TimerId, op: TimerOp },
    Quit,
}

#[derive(PartialEq, Eq, Debug)]
pub struct TimerId(pub usize);

#[derive(PartialEq, Eq, Debug)]
pub enum TimerOp {
    Start,
    Pause,
    Resume,
    Stop,
}
