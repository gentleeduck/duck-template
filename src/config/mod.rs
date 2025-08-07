pub mod config_structure;
pub mod fetch_config;

pub fn write_config(path: &str, config: &Config) {
  let json = serde_json::to_string_pretty(config).expect("Failed to serialize config");
  fs::write(path, json).expect("Failed to write config");
}

use std::{
  fs::{self, File},
  io::Read,
  process,
};

use url::Url;

use crate::{
  config::{config_structure::Config, fetch_config::curl_if_valid_url},
  logger::{log, LogLevel},
};

pub fn get_config(config: &String) -> Config {
  if config.is_empty() {
    log(LogLevel::Error, "🦆 No config file provided.");
    log(
      LogLevel::Info,
      "👉 Use `@duck-template init` to create a new template.",
    );
    log(
      LogLevel::Info,
      "👉 Use `@duck-template create` to create a new project.",
    );
    std::process::exit(1);
  }

  let str_buf = if Url::parse(config).is_ok() {
    match curl_if_valid_url(config) {
      Ok(body) => body,
      Err(e) => {
        log(LogLevel::Error, &format!("Failed to fetch config: {}", e));
        process::exit(1);
      },
    }
  } else {
    read_config(config)
  };

  serialize_config(&str_buf)
}

pub fn read_config(path: &String) -> String {
  let mut file = match File::open(path) {
    Ok(file) => file,
    Err(e) => {
      log(
        LogLevel::Error,
        &format!("Failed to read config {path}: {}", e),
      );
      process::exit(1);
    },
  };
  let mut buf = String::new();

  return match file.read_to_string(&mut buf) {
    Ok(_) => buf,
    Err(e) => {
      log(
        LogLevel::Error,
        &format!("Failed to read config {path}: {}", e),
      );
      process::exit(1);
    },
  };
}

pub fn serialize_config(str_buf: &String) -> config_structure::Config {
  return serde_json::from_str(str_buf).unwrap();
}
