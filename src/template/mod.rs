use std::collections::HashMap;

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
