use std::time::{Duration, Instant};
use InternalState::*;

pub struct Timer {
    label: String,
    duration: Duration,
    state: InternalState,
}

impl Timer {
    pub fn new(label: String, duration: Duration) -> Self {
        Self {
            label,
            duration,
            state: Reset,
        }
    }

    pub fn label(&self) -> &String {
        &self.label
    }

    pub fn duration(&self) -> Duration {
        self.duration
    }

    pub fn start(&mut self, now: Instant) {
        self.state = InternalState::Started { at: now }
    }

    pub fn resume(&mut self, now: Instant) {
        self.state = Started {
            at: now - self.elapsed(now),
        }
    }

    pub fn pause(&mut self, now: Instant) {
        self.state = Paused {
            elapsed: self.elapsed(now),
        }
    }

    pub fn reset(&mut self) {
        self.state = InternalState::Reset;
    }

    pub fn tick(&mut self, now: Instant) {
        match self.state {
            Reset | Paused { .. } | Stopped => {}
            Started { at } => {
                if now.saturating_duration_since(at) >= self.duration {
                    self.state = Stopped;
                }
            }
        }
    }

    pub fn state(&self) -> State {
        match self.state {
            Reset => State::Reset,
            Started { .. } => State::Started,
            Paused { .. } => State::Paused,
            Stopped => State::Stopped,
        }
    }

    pub fn elapsed(&self, now: Instant) -> Duration {
        match self.state {
            Reset => Duration::ZERO,
            Started { at } => now.saturating_duration_since(at),
            Paused { elapsed } => elapsed,
            Stopped => self.duration,
        }
    }

    pub fn elapsed_frac(&self, now: Instant) -> f32 {
        self.elapsed(now).as_secs_f32() / self.duration.as_secs_f32()
    }
}

#[derive(PartialEq, Eq, Debug)]
enum InternalState {
    Reset,
    Started { at: Instant },
    Paused { elapsed: Duration },
    Stopped,
}

#[derive(PartialEq, Eq, Debug)]
pub enum State {
    /// The timer is reset or has never been run.
    /// [`Timer::elapsed_frac()`] returns 0.0.
    Reset,
    Started,
    Paused,
    /// The timer is stopped automatically after the set period of time.
    /// [`Timer::elapsed_frac()`] returns 1.0.
    Stopped,
}
