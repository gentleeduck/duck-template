#[cfg(test)]
mod tests {
  use crate::help_command::execute_help_command;

  #[test]
  fn test_execute_help_command_runs_without_panic() {
    let cli_name = "duckcli";
    let cli_desc = "🦆 Your favorite CLI tool";
    let cli_version = "0.1.0";

    // We just want to ensure it runs without panicking
    execute_help_command(cli_name, cli_desc, cli_version);
  }
}
