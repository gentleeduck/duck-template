use std::collections::HashMap;

use crate::parse_commands::commands_structure::{ALL_COMMANDS, CommandHelp, FlagHelp};

pub fn find_command(name: &str) -> Option<&'static CommandHelp> {
  ALL_COMMANDS.iter().find(|cmd| cmd.command == name)
}

pub fn find_flag<'a>(flags: &'a [FlagHelp], name: &str) -> Option<&'a FlagHelp> {
  flags
    .iter()
    .find(|f| f.long == name || f.short.contains(&name))
}

pub fn get_command_value(
  long: &'static str,
  short: &'static str,
  raw_args: &HashMap<String, String>,
) -> String {
  raw_args
    .get(long)
    .cloned()
    .unwrap_or_else(|| raw_args.get(short).cloned().unwrap_or_default())
}
