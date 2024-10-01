#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialize our configuration reader
    let settings = config::Config::builder()
        // Add configuration values from a file name 'configuration.yaml'
        .add_source(
            config::File::new("/home/bilbo/Desktop/zero2prod/src/configuration.yaml", config::FileFormat::Yaml)
        )
        .build()?;
    // Try to convert the configuration values it read into our Settings type
    settings.try_deserialize::<Settings>()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        // Return string to connect to our database with
        format!("postgres://{}:{}@{}:{}/{}",
        self.username, self.password, self.host, self.port, self.database_name
        )
    }
}