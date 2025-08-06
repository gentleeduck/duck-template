use std::collections::HashMap;

use crate::{
  create_command::{create_structure::Create, parse_create_command::parse_create},
  init_command::{init_structure::Init, parase_init_command::parse_init},
  logger::{log, LogLevel},
};

pub const ALL_COMMANDS: &[(&str, &str)] = &[
  ("init", "Initializes a new project"),
  ("create", "Creates a new template"),
];

pub const ALL_FLAGS: &[(&str, &str)] = &[
  ("--help | -h", "Prints this help information"),
  ("--version | -h", "Shows the application version"),
];

#[derive(Debug)]
pub enum Command {
  Init(Init),
  Create(Create),
  Help,
  Version,
}

pub fn get_commands(args: &[String]) -> Vec<Command> {
  let mut commands = Vec::new();
  let mut i = 0;

  while i < args.len() {
    let arg = &args[i];

    // 1) Handle global flags immediately
    if matches!(arg.as_str(), "--help" | "-h") {
      commands.push(Command::Help);
      i += 1;
      continue;
    }
    if matches!(arg.as_str(), "--version" | "-v") {
      commands.push(Command::Version);
      i += 1;
      continue;
    }

    // 2) Handle subcommands
    match arg.as_str() {
      "init" => {
        let (cmd, next_idx) = parse_init(&args, i + 1);
        commands.push(Command::Init(cmd));
        i = next_idx;
      },
      "create" => {
        let (cmd, next_idx) = parse_create(&args, i + 1);
        commands.push(Command::Create(cmd));
        i = next_idx;
      },
      other if other.starts_with('-') => {
        // Unknown global flag
        log(LogLevel::Error, &format!("Unrecognized flag: {}", other));
        i += 1;
      },
      unknown_cmd => {
        log(
          LogLevel::Error,
          &format!("Unrecognized command: {}", unknown_cmd),
        );
        i += 1;
      },
    }
  }

  commands
}
