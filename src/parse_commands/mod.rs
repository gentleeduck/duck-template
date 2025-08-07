mod __test__;
pub mod commands_structure;
pub mod get_flag_value;

use crate::{
  create_command::create_structure::Create,
  create_variant_command::create_variant_structure::CreateVariant,
  init_command::init_structure::Init,
  logger::{log, LogLevel},
  parse_commands::{
    commands_structure::GLOBAL_FLAGS,
    get_flag_value::{find_command, find_flag},
  },
};

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Command {
  Init(Init),
  Create(Create),
  CreateVariant(CreateVariant),
  Help,
  Version,
  #[allow(dead_code)]
  Unknown(String),
}

pub fn get_commands(args: Vec<String>) -> Vec<Command> {
  let mut commands = Vec::new();
  let mut i = 0;

  while i < args.len() {
    let arg = &args[i];

    // Global flags
    match find_flag(GLOBAL_FLAGS, arg) {
      Some(flag) => {
        if flag.long == "--help" {
          commands.push(Command::Help);
        } else if flag.long == "--version" {
          commands.push(Command::Version);
        }
        i += 1;
        continue;
      },
      _ => (),
    }

    // Check for known command
    match find_command(arg) {
      Some(command_def) => {
        let mut cmd_args = HashMap::new();
        i += 1;
        while i < args.len() {
          let flag_or_arg = &args[i];
          if flag_or_arg.starts_with('-') {
            match find_flag(command_def.flags, flag_or_arg) {
              Some(flag_def) => {
                if flag_def.takes_value {
                  i += 1;
                  if i >= args.len() {
                    log(
                      LogLevel::Error,
                      &format!("👉 Expected value after flag `{}`", flag_or_arg),
                    );
                    break;
                  }
                  cmd_args.insert(flag_def.long.to_string(), args[i].clone());
                } else {
                  cmd_args.insert(flag_def.long.to_string(), "true".to_string());
                }
              },
              None => {
                log(
                  LogLevel::Warning,
                  &format!(
                    "⚠️ Unknown flag `{}` for command `{}`",
                    flag_or_arg, command_def.command
                  ),
                );
              },
            }
            i += 1;
          } else if find_command(flag_or_arg).is_some() {
            break; // stop parsing flags for this command
          } else {
            log(
              LogLevel::Warning,
              &format!(
                "⚠️ Unknown argument `{}` for command `{}`",
                flag_or_arg, command_def.command
              ),
            );
            i += 1;
          }
        }
        match command_def.command {
          "init" => commands.push(Command::Init(Init::from_args(&cmd_args))),
          "create" => commands.push(Command::Create(Create::from_args(&cmd_args))),
          "create-variant" => {
            commands.push(Command::CreateVariant(CreateVariant::from_args(&cmd_args)))
          },
          other => commands.push(Command::Unknown(other.to_string())),
        }
      },
      None => {
        log(LogLevel::Warning, &format!("⚠️ Unknown command `{}`", arg));
        i += 1;
      },
    }
  }

  commands
}
