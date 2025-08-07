use serde::Serialize;
use serde_json;
use std::{collections::HashMap, process};

use crate::logger::{log, LogLevel};

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
      variant: raw_args
        .get("--variant")
        .cloned()
        .unwrap_or_else(|| raw_args.get("-v").cloned().unwrap_or_default()),
      outdir: raw_args
        .get("--outdir")
        .cloned()
        .unwrap_or_else(|| raw_args.get("-o").cloned().unwrap_or_default()),
      config: raw_args
        .get("--config")
        .cloned()
        .unwrap_or_else(|| raw_args.get("-c").cloned().unwrap_or_default()),
      args: args_map,
    }
  }
}
