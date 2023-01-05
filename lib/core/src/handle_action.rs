use std::time::Instant;

use crate::{
    action::{Action, TimerId, TimerOp},
    state::State,
};

pub fn handle_action(action: Action, state: &mut State) -> Option<Action> {
    match action {
        Action::TimerAction {
            id: TimerId(idx),
            op,
        } => {
            let timer = &mut state.timers[idx];
            match op {
                TimerOp::Start => timer.start(Instant::now()),
                TimerOp::Pause => timer.pause(Instant::now()),
                TimerOp::Resume => timer.resume(Instant::now()),
                TimerOp::Stop => timer.reset(),
            }
            None
        }
        Action::Tick => {
            let now = Instant::now();
            for timer in &mut state.timers {
                timer.tick(now);
            }
            None
        }
        Action::Quit => Some(action),
    }
}
