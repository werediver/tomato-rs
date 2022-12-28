//! Keep actions non-clonable.

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
