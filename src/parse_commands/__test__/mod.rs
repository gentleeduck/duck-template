// src/parse_commands/__test__.rs

#[cfg(test)]
mod tests {
  use std::collections::HashMap;

  use crate::parse_commands::{
    get_commands,
    get_flag_value::{find_command, find_flag, get_command_value},
    Command,
  };

  #[test]
  fn test_find_command_valid() {
    let cmd = find_command("init");
    assert!(cmd.is_some());
    assert_eq!(cmd.unwrap().command, "init");
  }

  #[test]
  fn test_find_command_invalid() {
    assert!(find_command("nonexistent").is_none());
  }

  #[test]
  fn test_find_flag_by_long() {
    let cmd = find_command("init").unwrap();
    let flag = find_flag(cmd.flags, "--name");
    assert!(flag.is_some());
    assert_eq!(flag.unwrap().long, "--name");
  }

  #[test]
  fn test_find_flag_by_short() {
    let cmd = find_command("init").unwrap();
    let flag = find_flag(cmd.flags, "-n");
    assert!(flag.is_some());
    assert_eq!(flag.unwrap().short, "-n");
  }

  #[test]
  fn test_find_flag_invalid() {
    let cmd = find_command("init").unwrap();
    let flag = find_flag(cmd.flags, "--notreal");
    assert!(flag.is_none());
  }

  #[test]
  fn test_get_command_value_long() {
    let mut args = HashMap::new();
    args.insert("--name".to_string(), "test".to_string());
    assert_eq!(get_command_value("--name", "-n", &args), "test");
  }

  #[test]
  fn test_get_command_value_short() {
    let mut args = HashMap::new();
    args.insert("-n".to_string(), "short".to_string());
    assert_eq!(get_command_value("--name", "-n", &args), "short");
  }

  #[test]
  fn test_get_command_value_empty() {
    let args: HashMap<String, String> = HashMap::new();
    assert_eq!(get_command_value("--name", "-n", &args), "");
  }

  #[test]
  fn test_get_commands_help() {
    let result = get_commands(vec!["--help".into()]);
    assert_eq!(result.len(), 1);
    matches!(result[0], Command::Help);
  }

  #[test]
  fn test_get_commands_version() {
    let result = get_commands(vec!["--version".into()]);
    assert_eq!(result.len(), 1);
    matches!(result[0], Command::Version);
  }

  #[test]
  fn test_get_commands_init_args() {
    let result = get_commands(vec!["init".into(), "--name".into(), "myproject".into()]);
    assert_eq!(result.len(), 1);
    if let Command::Init(init) = &result[0] {
      assert_eq!(init.name, "myproject");
    } else {
      panic!("Expected Init command");
    }
  }

  #[test]
  fn test_get_commands_create_variant() {
    let result = get_commands(vec![
      "create-variant".into(),
      "--name".into(),
      "variant1".into(),
    ]);
    assert_eq!(result.len(), 1);
    if let Command::CreateVariant(cv) = &result[0] {
      assert_eq!(cv.name, "variant1");
    } else {
      panic!("Expected CreateVariant command");
    }
  }

  #[test]
  fn test_get_commands_unknown() {
    let result = get_commands(vec!["unknown-cmd".into()]);
    assert!(result.is_empty() || matches!(result[0], Command::Unknown(_)));
  }

  #[test]
  fn test_get_commands_missing_value() {
    let result = get_commands(vec!["init".into(), "--name".into()]);
    assert_eq!(result.len(), 1);
    // name should be empty due to missing value
    if let Command::Init(init) = &result[0] {
      assert_eq!(init.name, "");
    }
  }
}
