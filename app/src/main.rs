use std::fs::read_to_string;
use std::io;
use std::str::FromStr;
use std::time::Duration;

use conf::Conf;
use core::{action::Action, state::State, Core};

mod gui_imgui;
use gui_imgui as gui;


use gui::Gui;

fn load_conf() -> io::Result<Conf> {
    let data = read_to_string("tomato.toml")?;
    let conf =
        Conf::from_str(data.as_ref()).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(conf)
}

fn main() {
    let conf = load_conf().unwrap();
    let state = State {
        timers: conf
            .timers
            .iter()
            .map(|t| {
                core::timer::Timer::new(
                    t.label.clone(),
                    Duration::from_secs_f32(t.duration as f32 * 60.0),
                )
            })
            .collect::<Vec<_>>(),
    };
    let mut core = Core::new(state);

    let mut gui = Gui::new(&conf, core.state());

    'main: loop {
        let actions = gui.update(&conf, core.state());
        for action in actions {
            if let Some(action) = core.handle(action) {
                match action {
                    Action::Quit => break 'main,
                    _ => {}
                }
            }
        }
    }
}
