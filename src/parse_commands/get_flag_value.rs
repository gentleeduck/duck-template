use crate::parse_commands::commands_structure::{CommandHelp, FlagHelp, ALL_COMMANDS};

pub fn find_command(name: &str) -> Option<&'static CommandHelp> {
  ALL_COMMANDS.iter().find(|cmd| cmd.command == name)
}

pub fn find_flag<'a>(flags: &'a [FlagHelp], name: &str) -> Option<&'a FlagHelp> {
  flags
    .iter()
    .find(|f| f.long == name || f.short.contains(&name))
}
