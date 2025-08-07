#[cfg(test)]
mod tests {
  use crate::create_variant_command::create_variant_command;
  use crate::create_variant_command::create_variant_structure::CreateVariant;
  use std::fs::{self, File};
  use std::io::Write;
  use tempfile::tempdir;

  fn create_temp_config_file() -> (String, tempfile::TempDir) {
    let config = r#"
        {
            "$schema": "https://example.com/schema.json",
            "name": "duck-sample",
            "version": "0.1.0",
            "description": "Sample config",
            "variants": []
        }
        "#;

    let dir = tempdir().unwrap();
    let file_path = dir.path().join("config.json");
    let mut file = File::create(&file_path).unwrap();
    file.write_all(config.as_bytes()).unwrap();

    (file_path.to_str().unwrap().to_string(), dir)
  }

  fn setup_source_dir() -> (tempfile::TempDir, String) {
    let dir = tempdir().unwrap();
    let path_string = dir.path().to_str().unwrap().to_string(); // clone before move
    let file_path = dir.path().join("example.txt");
    fs::write(&file_path, "Hello World").unwrap();
    (dir, path_string)
  }

  #[test]
  fn test_create_variant_command_adds_variant_to_config() {
    // Prepare temp config
    let (config_path, _config_dir) = create_temp_config_file();

    // Prepare dummy source directory
    let (_source_dir, source_path) = setup_source_dir();

    // Construct CreateVariant input
    let create_variant = CreateVariant {
      name: "custom-variant".to_string(),
      source: source_path.clone(),
      config: config_path.clone(),
      description: "This is a test variant.".to_string(),
    };

    // Run command
    let result = create_variant_command(&create_variant);
    assert!(result.is_ok());

    // Read updated config and assert variant added
    let updated_config_str = fs::read_to_string(config_path).unwrap();
    let updated_config: crate::config::config_structure::Config =
      serde_json::from_str(&updated_config_str).unwrap();

    assert_eq!(updated_config.variants.len(), 1);
    let added_variant = &updated_config.variants[0];
    assert_eq!(added_variant.name, "custom-variant");
    assert_eq!(added_variant.description, "This is a test variant.");
    assert_eq!(added_variant.src.len(), 1); // should contain the parsed source folder
  }
}
