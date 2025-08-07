use crate::parse_commands::commands_structure::{FlagHelp, ALL_COMMANDS, GLOBAL_FLAGS};

pub fn execute_help_command(cli_name: &str, cli_description: &str, cli_version: &str) {
  let max_command_width = ALL_COMMANDS
    .iter()
    .map(|cmd| cmd.command.len())
    .max()
    .unwrap_or(0);

  let all_flags = GLOBAL_FLAGS
    .iter()
    .chain(ALL_COMMANDS.iter().flat_map(|cmd| cmd.flags.iter()));

  let max_flag_width = all_flags
    .clone()
    .map(|flag| format_flag_with_aliases(flag).len())
    .max()
    .unwrap_or(0);

  // Header
  println!("{} - {}", cli_name, cli_description);
  println!("Version: {}\n", cli_version);

  // Commands
  println!("COMMANDS:");
  for cmd in ALL_COMMANDS {
    println!(
      "    {:<width$}{}",
      cmd.command,
      cmd.description,
      width = max_command_width + 2
    );
  }
  println!();

  // Global flags
  if !GLOBAL_FLAGS.is_empty() {
    println!("GLOBAL FLAGS:");
    for flag in GLOBAL_FLAGS {
      let display_flag = format_flag_with_aliases(flag);
      println!(
        "    {:<width$}{}",
        display_flag,
        flag.description,
        width = max_flag_width + 2
      );
    }
    println!();
  }

  // Command-specific flags
  for command in ALL_COMMANDS {
    if !command.flags.is_empty() {
      println!("FLAGS for '{}':", command.command);
      for flag in command.flags {
        let display_flag = format_flag_with_aliases(flag);
        println!(
          "    {:<width$}{}",
          display_flag,
          flag.description,
          width = max_flag_width + 2
        );
      }
      println!();
    }
  }

  // Examples
  println!("EXAMPLES:");
  println!("    {} init --name my-app", cli_name);
  println!("    {} create -v rust-template", cli_name);
  println!("    {} --help\n", cli_name);
}

/// Formats a flag's short and long form into a string, with `<value>` if needed
fn format_flag_with_aliases(flag: &FlagHelp) -> String {
  let base = format!("{} | {}", flag.short, flag.long);
  if flag.takes_value {
    format!("{} <value>", base)
  } else {
    base
  }
}
