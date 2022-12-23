use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Conf {
    #[serde(default = "default::title")]
    pub title: String,
    #[serde(default = "default::window_width")]
    pub window_width: u32,
    #[serde(default = "default::bar_height")]
    pub bar_height: u32,
    #[serde(default = "default::timers")]
    pub timers: Vec<Timer>,
}

mod default {
    use super::Timer;

    pub fn title() -> String {
        "🍅 Tomato".to_owned()
    }

    pub fn window_width() -> u32 {
        320
    }

    pub fn bar_height() -> u32 {
        20
    }

    pub fn timers() -> Vec<Timer> {
        vec![
            Timer {
                label: "Work".to_owned(),
                duration: 25,
            },
            Timer {
                label: "Br".to_owned(),
                duration: 5,
            },
            Timer {
                label: "Break".to_owned(),
                duration: 15,
            },
        ]
    }
}

impl FromStr for Conf {
    type Err = toml::de::Error;

    fn from_str(s: &str) -> Result<Conf, Self::Err> {
        toml::from_str(s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Timer {
    pub label: String,
    /// Duration in minutes.
    pub duration: u32,
}
