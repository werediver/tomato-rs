pub mod action;
mod handle_action;
pub mod state;
pub mod timer;

use action::Action;
use handle_action::handle_action;
use state::State;

pub struct Core {
    state: State,
}

impl Core {
    pub fn new(app_state: State) -> Self {
        Self { state: app_state }
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn handle(&mut self, action: Action) -> Option<Action> {
        handle_action(action, &mut self.state)
    }
}
