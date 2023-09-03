use std::fs;
use serde::Deserialize;
use lazy_static::lazy_static;

#[derive(Deserialize)]
pub struct Config {
    pub bot: BotConfig,
    pub modules: ModulesConfig,
}

#[derive(Deserialize)]
pub struct BotConfig {
    pub token: String,
    pub cache_time: u32,
}

#[derive(Deserialize)]
pub struct ModulesConfig {
    pub gengshuang_api: String,
}

pub fn read_config() -> Config {
    let input = fs::read_to_string("config.yml").unwrap();
    serde_yaml::from_str(&input).unwrap()
}

lazy_static! {
    pub static ref CONFIG: Config = read_config();
}