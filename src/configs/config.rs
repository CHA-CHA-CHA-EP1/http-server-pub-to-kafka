use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub server_port: u16,
}

pub fn get_env() -> Result<Config, envy::Error> {
    return envy::from_env::<Config>();
}
