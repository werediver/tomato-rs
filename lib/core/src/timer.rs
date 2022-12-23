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
            state: Stopped,
        }
    }

    pub fn label(&self) -> &String {
        &self.label
    }

    pub fn duration(&self) -> Duration {
        self.duration
    }

    pub fn start(&mut self) {
        self.state = InternalState::Started { at: Instant::now() }
    }

    pub fn resume(&mut self) {
        self.state = Started {
            at: Instant::now() - self.elapsed(),
        }
    }

    pub fn pause(&mut self) {
        self.state = Paused {
            elapsed: self.elapsed(),
        }
    }

    pub fn stop(&mut self) {
        self.state = InternalState::Stopped
    }

    pub fn state(&self) -> State {
        match self.state {
            Stopped => State::Stopped,
            Started { .. } => State::Started,
            Paused { .. } => State::Paused,
        }
    }

    pub fn elapsed(&self) -> Duration {
        match self.state {
            Stopped => Duration::ZERO,
            Started { at } => Instant::now() - at,
            Paused { elapsed } => elapsed,
        }
    }

    pub fn elapsed_frac(&self) -> f32 {
        self.elapsed().as_secs_f32() / self.duration.as_secs_f32()
    }
}

#[derive(PartialEq, Eq, Debug)]
enum InternalState {
    Stopped,
    Started { at: Instant },
    Paused { elapsed: Duration },
}

#[derive(PartialEq, Eq, Debug)]
pub enum State {
    Stopped,
    Started,
    Paused,
}
