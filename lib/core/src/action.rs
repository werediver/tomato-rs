#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Action {
    TimerAction { id: TimerId, op: TimerOp },
    Quit,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct TimerId(pub usize);

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum TimerOp {
    Start,
    Pause,
    Resume,
    Stop,
}
