use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Conf {
    pub title: Option<String>,
    #[serde(rename = "timer")]
    pub timers: Vec<Timer>,
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
    pub duration: i32,
}
