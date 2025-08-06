use crate::{
  init_command::init_structure::Init,
  logger::{log, LogLevel},
  parse_commands::get_flag_value::get_flag_value,
};

pub fn parse_init(args: &[String], mut idx: usize) -> (Init, usize) {
  let mut name = String::new();

  while idx < args.len() {
    match args[idx].as_str() {
      "--name" | "-n" => {
        idx += 1;
        if let Some(val) = get_flag_value(args, idx, "--name") {
          name = val;
        }
        idx += 1;
      },
      next if next == "create" || next == "init" || next.starts_with('-') => break,
      _ => {
        log(
          LogLevel::Error,
          &format!("👉 Unrecognized init flag: {}", args[idx]),
        );
        idx += 1;
      },
    }
  }

  (Init { name }, idx)
}
