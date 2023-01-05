use crate::{
    action::{Action, TimerId, TimerOp},
    state::State,
};

pub fn handle_action(action: Action, app_state: &mut State) -> Option<Action> {
    match action {
        Action::TimerAction {
            id: TimerId(idx),
            op,
        } => {
            let timer = &mut app_state.timers[idx];
            match op {
                TimerOp::Start => timer.start(),
                TimerOp::Pause => timer.pause(),
                TimerOp::Resume => timer.resume(),
                TimerOp::Stop => timer.stop(),
            }
            None
        }
        Action::Quit => Some(action),
    }
}
