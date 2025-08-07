mod __test__;
pub enum LogLevel {
  Error,
  Warning,
  Success,
  Info,
}

pub fn log(level: LogLevel, message: &str) {
  let (prefix, color) = match level {
    LogLevel::Error => ("[ERROR]", "\x1b[31m"),  // Red
    LogLevel::Warning => ("[WARN]", "\x1b[33m"), // Yellow
    LogLevel::Success => ("[OK]", "\x1b[32m"),   // Green
    LogLevel::Info => ("[INFO]", "\x1b[34m"),    // Blue
  };

  println!("{}{} {}\x1b[0m", color, prefix, message);
}
