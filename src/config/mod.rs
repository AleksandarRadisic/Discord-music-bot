use serde::Deserialize;
use serenity::prelude::TypeMapKey;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub prefix: String,
    pub token: String
}

impl TypeMapKey for Config {
    type Value = Config;
}

pub fn load_config() -> Config {
    let file = File::open("config.json").expect("Unable to open config file");
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader).expect("Unable to parse config file");
    config
}