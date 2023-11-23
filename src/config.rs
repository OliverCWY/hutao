use serde::{Serialize, Deserialize};
use serde_default::DefaultFromSerde;
use winput::Vk;

#[derive(Serialize, Deserialize, DefaultFromSerde)]
pub struct Config {
    #[serde(default = "default_title")]
    pub target_window_title: String,
    #[serde(default = "default_key")]
    pub key: Vk,
    #[serde(default = "default_max_title_length")]
    pub max_title_length: i32,
    #[serde(default = "default_check_interval")]
    pub check_interval: u64,
    #[serde(default = "default_na_durations")]
    pub na_durations: Vec<u64>,
    #[serde(default = "default_na_threshold")]
    pub na_threshold: u64,
    #[serde(default = "default_jump_duration")]
    pub jump_duration: u64,
    #[serde(default = "default_dodge_duration")]
    pub dodge_duration: u64,
    #[serde(default)]
    pub use_dodge: bool
}

fn default_title() -> String {
    "原神".to_string()
}

fn default_key() -> Vk {
    Vk::Oem3
}

fn default_max_title_length() -> i32 {
    256
}

fn default_check_interval() -> u64 {
    100
}

fn default_na_durations() -> Vec<u64> {
    vec![160, 150]
}

fn default_na_threshold() -> u64 {
    400
}

fn default_jump_duration() -> u64 {
    600
}

fn default_dodge_duration() -> u64 {
    300
}