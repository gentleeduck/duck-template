mod config;
mod create_command;
mod help_command;
mod init_command;
mod logger;
mod parse_commands;
mod template;

use crate::create_command::create_command;
use crate::help_command::execute_help_command;
use crate::init_command::init_command;
use crate::logger::log;
use crate::parse_commands::{get_commands, Command};

const CLI_NAME: &str = "@duck-template";
const CLI_VERSION: &str = "1.0.0";
const CLI_DESCRIPTION: &str = "Generate and manage project templates with ease";

fn main() {
  let mut args: Vec<String> = std::env::args().skip(1).collect();
  let options = get_commands(args);

  if options.is_empty() {
    eprintln!(
      "❌ No command provided. Use `{}` help to see available commands.",
      CLI_NAME
    );
    std::process::exit(1);
  }

  for command in options {
    match command {
      Command::Init(init) => {
        // init_command(&init);
      },
      Command::Version => {
        println!("{} V{}", CLI_NAME, CLI_VERSION);
      },
      Command::Create(create) => {
        // create_command(create);
      },
      Command::Help => {
        execute_help_command(CLI_NAME, CLI_DESCRIPTION, CLI_VERSION);
      },
      _ => {},
    }
  }
}
