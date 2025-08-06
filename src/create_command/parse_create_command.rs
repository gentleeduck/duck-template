use std::collections::HashMap;

use crate::{
  create_command::create_structure::Create,
  logger::{log, LogLevel},
};

/// Parse 'create' subcommand options
pub fn parse_create(args: &[String], mut idx: usize) -> (Create, usize) {
  let mut variant = String::new();
  let mut outdir = String::from("./");
  let mut cli_args = HashMap::new();
  let mut config = String::new();

  while idx < args.len() {
    match args[idx].as_str() {
      "--variant" | "-v" => {
        idx += 1;
        if let Some(val) = args.get(idx) {
          variant = val.clone();
        }
        idx += 1;
      },
      "--args" | "-a" => {
        idx += 1;
        if let Some(arg_pair) = args.get(idx) {
          let parts: Vec<&str> = arg_pair.split('=').collect();
          if parts.len() == 2 {
            cli_args.insert(parts[0].to_string(), parts[1].to_string());
          } else {
            log(
              LogLevel::Error,
              &format!("Invalid args format, expected key=value: {}", arg_pair),
            );
          }
        }
        idx += 1;
      },
      "--outdir" | "-d" => {
        idx += 1;
        if let Some(val) = args.get(idx) {
          outdir = val.clone();
        }
        idx += 1;
      },
      "--config" | "-c" => {
        idx += 1;
        if let Some(val) = args.get(idx) {
          config = val.clone();
        }
        idx += 1;
      },
      next if next == "create" || next == "init" || next.starts_with('-') => break,
      _ => idx += 1,
    }
  }

  (
    Create {
      variant,
      outdir,
      args: cli_args,
      config,
    },
    idx,
  )
}
