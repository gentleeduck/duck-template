mod config;
mod create_command;
mod create_variant_command;
mod hasher;
mod help_command;
mod init_command;
mod logger;
mod parse_commands;
mod template;

use crate::{
  create_command::create_command,
  create_variant_command::create_variant_command,
  help_command::execute_help_command,
  init_command::init_command,
  logger::{LogLevel, log},
  parse_commands::{Command, get_commands},
};

const CLI_NAME: &str = "@duck-template";
const CLI_VERSION: &str = "0.1.8";
const CLI_DESCRIPTION: &str = "Generate and manage project templates with ease";

fn main() {
  let args: Vec<String> = std::env::args().skip(1).collect();
  let options = get_commands(args);

  if options.is_empty() {
    eprintln!(
      "❌ No command provided. Use `{}` help to see available commands.",
      CLI_NAME
    );
    std::process::exit(1)
  }

  for command in options {
    match command {
      Command::Init(init) => {
        init_command(&init);
      },
      Command::Version => {
        println!("{} V{}", CLI_NAME, CLI_VERSION);
      },
      Command::Create(create) => {
        create_command(&create);
      },
      Command::CreateVariant(create_variant) => {
        if let Err(err) = create_variant_command(&create_variant) {
          log(LogLevel::Error, &err.to_string());
          std::process::exit(1);
        }
      },
      Command::Help => {
        execute_help_command(CLI_NAME, CLI_DESCRIPTION, CLI_VERSION);
      },
      _ => {},
    }
  }
}
