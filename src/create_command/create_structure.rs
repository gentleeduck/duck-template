use serde::Serialize;
use serde_json;
use std::{collections::HashMap, process};

use crate::{
  logger::{log, LogLevel},
  parse_commands::{commands_structure::FlagHelp, get_flag_value::get_command_value},
};

#[derive(Debug, Clone, Serialize)]
pub struct Create {
  pub variant: String,
  pub outdir: String,
  pub args: HashMap<String, String>,
  pub config: String,
}

impl Create {
  pub fn from_args(raw_args: &HashMap<String, String>) -> Self {
    let args_json_str = raw_args
      .get("--args")
      .or_else(|| raw_args.get("-a"))
      .cloned()
      .unwrap_or_else(|| "{}".to_string());

    let args_map: HashMap<String, String> =
      serde_json::from_str(&args_json_str).unwrap_or_else(|err| {
        log(
          LogLevel::Info,
          &format!("❌ Failed to parse --args JSON: {}", err),
        );
        process::exit(1);
      });

    Self {
      variant: get_command_value("--variant", "-v", raw_args),
      outdir: get_command_value("--outidr", "-o", raw_args),
      config: get_command_value("--config", "-c", raw_args),
      args: args_map,
    }
  }
}

pub const CREATE_FLAGS: &[FlagHelp] = &[
  FlagHelp {
    long: "--variant",
    short: "-v",
    description: "Template variant",
    takes_value: true,
  },
  FlagHelp {
    long: "--outdir",
    short: "-d",
    description: "Output directory",
    takes_value: true,
  },
  FlagHelp {
    long: "--config",
    short: "-c",
    description: "Path to config file",
    takes_value: true,
  },
  FlagHelp {
    long: "--args",
    short: "-a",
    description: "Extra arguments (key=value)",
    takes_value: true,
  },
];
