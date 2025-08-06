use crate::create_command::create_structure::Create;
use crate::logger::{log, LogLevel};
use crate::parse_commands::get_flag_value::get_flag_value;
use std::collections::HashMap;

pub fn parse_create(args: &[String], mut idx: usize) -> (Create, usize) {
  let mut variant = String::new();
  let mut outdir = String::new();
  let mut cli_args = HashMap::new();
  let mut config = String::new();

  while idx < args.len() {
    match args[idx].as_str() {
      "--variant" | "-v" => {
        idx += 1;
        if let Some(val) = get_flag_value(args, idx, "--variant") {
          variant = val;
        }
        idx += 1;
      },

      "--outdir" | "-d" => {
        idx += 1;
        if let Some(val) = get_flag_value(args, idx, "--outdir") {
          outdir = val;
        }
        idx += 1;
      },

      "--config" | "-c" => {
        idx += 1;
        if let Some(val) = get_flag_value(args, idx, "--config") {
          config = val;
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
        } else {
          log(LogLevel::Error, "Expected key=value pair after '--args'");
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
