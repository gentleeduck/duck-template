use crate::parse_commands::{ALL_COMMANDS, ALL_FLAGS};

pub fn execute_help_command(cli_name: &str, cli_description: &str, cli_version: &str) {
  // Calculate maximum width for command names
  let max_command_width = ALL_COMMANDS
    .iter()
    .map(|(cmd, _)| cmd.len())
    .max()
    .unwrap_or(0);

  // Calculate maximum width for flag names
  let max_flag_width = ALL_FLAGS
    .iter()
    .map(|(flag, _)| flag.len())
    .max()
    .unwrap_or(0);

  // Generate the COMMANDS section
  let commands_section = ALL_COMMANDS
    .iter()
    .map(|(cmd, desc)| format!("    {:<width$}{}", cmd, desc, width = max_command_width + 2))
    .collect::<Vec<_>>()
    .join("\n");

  // Generate the FLAGS section
  let flags_section = if !ALL_FLAGS.is_empty() {
    ALL_FLAGS
      .iter()
      .map(|(flag, desc)| format!("    {:<width$}{}", flag, desc, width = max_flag_width + 2))
      .collect::<Vec<_>>()
      .join("\n")
  } else {
    String::from("    (none)")
  };

  // Example section
  let examples = format!("    {0} init\n    {0} version\n    {0} help", cli_name);

  // Final message
  let message = format!(
    "{cli} - {cli_description}

VERSION: v{cli_version}

USAGE:
    {cli} <COMMAND> [OPTIONS]

COMMANDS:
{commands}

FLAGS:
{flags}

EXAMPLES:
{examples}
",
    cli = cli_name,
    cli_description = cli_description,
    cli_version = cli_version,
    commands = commands_section,
    flags = flags_section,
    examples = examples
  );

  println!("{}", message);
}
