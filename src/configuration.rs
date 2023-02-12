use config::{Config, ConfigError, File, FileFormat};

#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize, Debug)]
pub struct DatabaseSettings {
    pub name: String,
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
}

pub fn get_config() -> Result<Settings, ConfigError> {
    let settings = Config::builder()
        .add_source(File::new("configuration.yaml", FileFormat::Yaml))
        .build();

    settings?.try_deserialize::<Settings>()
}
