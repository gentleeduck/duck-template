use std::{collections::HashMap, fs, path::Path, process};

use crate::{
  config::config_structure::{File, Folder, Source},
  logger::{log, LogLevel},
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

pub fn parse_source(path: &Path) -> Source {
  if path.is_file() {
    // Read file content or use empty string if unreadable
    let content = fs::read_to_string(path).unwrap_or_else(|_| "".to_string());

    Source::File(File {
      path: path.to_string_lossy().to_string(),
      content,
      args: Some(vec![]),
    })
  } else if path.is_dir() {
    let mut children = vec![];

    // Recursively parse children
    for entry in fs::read_dir(path).expect("Failed to read folder") {
      if let Ok(entry) = entry {
        let child_path = entry.path();
        // This call will return either File or Folder depending on type
        let child_source = parse_source(&child_path);
        children.push(child_source);
      }
    }

    Source::Folder(Folder {
      path: path.to_string_lossy().to_string(),
      children,
    })
  } else {
    log(LogLevel::Error, &format!("❌ Invalid path: {:?}", path));
    process::exit(1);
  }
}
