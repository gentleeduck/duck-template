#[cfg(test)]
mod tests {
  use crate::config::config_structure::Config;
  use crate::init_command::init_command;
  use crate::init_command::init_structure::Init;

  use std::fs;
  use std::path::Path;

  #[test]
  fn test_init_command_creates_template_file() {
    let project_name = "test_project";
    let init = Init {
      name: project_name.to_string(),
    };

    let expected_file = format!("{}-template.json", project_name);

    // Clean up before test in case of leftover file
    if Path::new(&expected_file).exists() {
      fs::remove_file(&expected_file).unwrap();
    }

    // Run the command
    init_command(&init);

    // Assert file exists
    assert!(
      Path::new(&expected_file).exists(),
      "Template file was not created"
    );

    // Assert file is not empty
    let contents = fs::read_to_string(&expected_file).expect("Failed to read created file");
    assert!(
      contents.contains("\"name\": \"test_project\""),
      "Template JSON missing expected content"
    );

    // Optionally deserialize and assert specific fields using serde_json
    let config: Config = serde_json::from_str(&contents).expect("Invalid JSON structure");
    assert_eq!(config.name, project_name);
    assert_eq!(config.version, "0.1.0");

    // Clean up
    fs::remove_file(&expected_file).unwrap();
  }
}
