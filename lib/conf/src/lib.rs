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
                name: "Work".to_owned(),
                duration: 25,
            },
            Timer {
                name: "Br".to_owned(),
                duration: 5,
            },
            Timer {
                name: "Break".to_owned(),
                duration: 15,
            },
        ]
    }
}

impl Conf {
    pub fn from_str(s: impl AsRef<str>) -> Result<Conf, toml::de::Error> {
        toml::from_str(s.as_ref())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Timer {
    pub name: String,
    /// Duration in minutes.
    pub duration: u32,
}
