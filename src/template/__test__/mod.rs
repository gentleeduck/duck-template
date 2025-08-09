#[cfg(test)]
mod tests {
  use std::{
    collections::HashMap,
    fs::{self, create_dir},
    io::Write,
  };

  use tempfile::{NamedTempFile, tempdir};

  use crate::{
    config::config_structure::Source,
    template::{parse_source, replace_args},
  };

  #[test]
  fn test_replace_args_basic() {
    let mut args = HashMap::new();
    args.insert("name".to_string(), "duck".to_string());
    args.insert("lang".to_string(), "rust".to_string());

    let file_args = vec!["{name}".to_string(), "{lang}".to_string()];
    let content = "Hello, {name}! Welcome to {lang}.";

    let result = replace_args(content, &args, &file_args);
    assert_eq!(result, "Hello, duck! Welcome to rust.");
  }

  #[test]
  fn test_replace_args_missing_key() {
    let mut args = HashMap::new();
    args.insert("name".to_string(), "duck".to_string());

    let file_args = vec!["{name}".to_string(), "{lang}".to_string()];
    let content = "Hello, {name}! Welcome to {lang}.";

    let result = replace_args(content, &args, &file_args);
    assert_eq!(result, "Hello, duck! Welcome to {lang}.");
  }

  #[test]
  fn test_parse_source_file() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "duck = true").unwrap();

    let path = file.path();
    let root = path.parent().unwrap();

    let source = parse_source(path, root, &vec![]);
    match source {
      Source::File(f) => {
        assert!(
          f.path
            .ends_with(file.path().file_name().unwrap().to_str().unwrap())
        );
        assert!(f.content.contains("duck = true"));
        assert_eq!(f.args, Some(vec![]));
      },
      _ => panic!("Expected Source::File"),
    }
  }

  #[test]
  fn test_parse_source_empty_folder() {
    let dir = tempdir().unwrap();
    let source = parse_source(dir.path(), dir.path(), &vec![]);

    match source {
      Source::Folder(folder) => {
        assert_eq!(folder.path, dir.path().to_str().unwrap()); // ✅ now matches absolute
        assert_eq!(folder.children.len(), 0);
      },
      _ => panic!("Expected Source::Folder"),
    }
  }

  #[test]
  fn test_parse_source_nested_structure() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Create nested folders and files
    let nested_dir = root.join("config");
    let _ = create_dir(&nested_dir);
    let file_path = nested_dir.join("settings.toml");
    fs::write(&file_path, "key = \"value\"").unwrap();

    let source = parse_source(root, root, &vec![]);
    match source {
      Source::Folder(folder) => {
        assert_eq!(folder.path, root.to_str().unwrap());

        let config_folder = folder
          .children
          .iter()
          .find(|c| match c {
            Source::Folder(f) => f.path.ends_with("config"), // ✅ changed
            _ => false,
          })
          .expect("Missing config folder");

        match config_folder {
          Source::Folder(f) => {
            assert_eq!(f.children.len(), 1);
            match &f.children[0] {
              Source::File(f) => {
                assert!(f.path.ends_with("config/settings.toml")); // ✅ changed
                assert!(f.content.contains("key = \"value\""));
              },
              _ => panic!("Expected file in config folder"),
            }
          },
          _ => panic!("Expected config folder"),
        }
      },
      _ => panic!("Expected root folder"),
    }
  }

  #[test]
  fn test_parse_source_skips_hidden_files() {
    let dir = tempdir().unwrap();
    let hidden_file = dir.path().join(".hidden");
    fs::write(&hidden_file, "secret").unwrap();

    let source = parse_source(dir.path(), dir.path(), &vec![]);
    match source {
      Source::Folder(folder) => {
        assert_eq!(folder.children.len(), 0);
      },
      _ => panic!("Expected folder"),
    }
  }
}
