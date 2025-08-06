use std::process;

use crate::logger::{log, LogLevel};

pub fn get_flag_value(args: &[String], idx: usize, flag: &str) -> Option<String> {
  match args.get(idx) {
    Some(val) if val.starts_with('-') => {
      log(
        LogLevel::Error,
        &format!(
          "Expected a value after '{}', but got another flag '{}'",
          flag, val
        ),
      );
      process::exit(1);
    },
    Some(val) => Some(val.clone()),
    None => {
      log(
        LogLevel::Error,
        &format!(
          "Expected a value after '{}', but nothing was provided",
          flag
        ),
      );
      process::exit(1);
    },
  }
}
