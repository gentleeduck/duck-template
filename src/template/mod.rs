mod __test__;
use std::{collections::HashMap, fs, path::Path, process};

use crate::{
  config::config_structure::{File, Folder, Source},
  logger::{LogLevel, log},
};

pub fn replace_args(content: &str, args: &HashMap<String, String>, file_args: &[String]) -> String {
  let mut result = content.to_string();

  for key in file_args {
    let trimmed = key.trim_matches(|c: char| c == '{' || c == '}' || c.is_whitespace());
    if let Some(value) = args.get(trimmed) {
      result = result.replace(key, value);
    }
  }

  result
}

pub fn parse_source(path: &Path, root: &Path) -> Source {
  let rel_path = path.strip_prefix(root).unwrap_or(path);
  let rel_path_str = if rel_path.as_os_str().is_empty() {
    "".to_string()
  } else {
    rel_path.to_string_lossy().to_string()
  };

  if path.is_file() {
    let content = fs::read_to_string(path).unwrap_or_else(|_| "".to_string());

    Source::File(File {
      path: rel_path_str,
      content,
      args: Some(vec![]),
    })
  } else if path.is_dir() {
    let mut children = vec![];

    for entry in fs::read_dir(path).expect("Failed to read folder") {
      if let Ok(entry) = entry {
        let child_path = entry.path();

        // Skip hidden files/folders (optional)
        if let Some(name) = child_path.file_name().and_then(|n| n.to_str()) {
          if name.starts_with('.') {
            continue;
          }
        }

        let child_source = parse_source(&child_path, root);
        children.push(child_source);
      }
    }

    Source::Folder(Folder {
      path: rel_path_str,
      children,
    })
  } else {
    log(LogLevel::Error, &format!("❌ Invalid path: {:?}", path));
    process::exit(1);
  }
}
