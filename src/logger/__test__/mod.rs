#[cfg(test)]
mod tests {
  use crate::logger::{log, LogLevel};

  #[test]
  fn test_log_levels() {
    log(LogLevel::Error, "This is an error");
    log(LogLevel::Warning, "This is a warning");
    log(LogLevel::Success, "This is a success");
    log(LogLevel::Info, "This is an info");

    // Note: This test doesn't assert anything.
    // It's just for visual inspection of colored output.
    // To test printed output properly, you'd need to capture stdout.
  }
}
