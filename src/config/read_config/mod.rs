use std::{fs::File, io::Read, process};

use crate::{
  config::config_structure,
  logger::{log, LogLevel},
};

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
