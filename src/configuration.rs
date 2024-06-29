#[derive(serde::Deserialize)]
pub struct Settings {
  pub readline: ReadlineSettings,
  pub history_file: String,
}

#[derive(serde::Deserialize)]
pub struct ReadlineSettings {
    pub log_level: String,
    pub prompt: String,
    pub version: String,
    pub save_history: bool
}

impl ReadlineSettings {
  pub fn log_level(&self) -> String {
    format!(
      "level:{}", 
      self.log_level
    )
  }
}
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
  // Initialise our configuration reader
  let settings = config::Config::builder()
      // Add configuration values from a file named `configuration.toml`.
      .add_source(config::File::new("configuration.toml", config::FileFormat::Toml))
      .build()?;
  // Try to convert the configuration values it read into
  // our Settings type
  settings.try_deserialize::<Settings>()
}