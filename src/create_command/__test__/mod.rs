#[cfg(test)]
mod tests {
  use std::{collections::HashMap, fs, fs::File, io::Write};

  use tempfile::tempdir;

  use crate::create_command::{create_command, create_structure::Create};

  fn sample_config_json() -> String {
    r#"
        {
            "$schema": "https://example.com/schema.json",
            "name": "duck-sample",
            "version": "0.1.0",
            "description": "Sample template",
            "outdir": "./ignored",
            "variants": [
                {
                    "name": "basic",
                    "description": "Basic setup",
                    "src": [
                        {
                            "type": "file",
                            "path": "hello.txt",
                            "content": "Hello, {{name}}!",
                            "args": ["name"]
                        },
                        {
                            "type": "folder",
                            "path": "src",
                            "children": [
                                {
                                    "type": "file",
                                    "path": "src/main.rs",
                                    "content": "fn main() { println!(\"{{greet}}\"); }",
                                    "args": ["greet"]
                                }
                            ]
                        }
                    ]
                }
            ]
        }
        "#
    .to_string()
  }

  fn write_temp_config(json: &str) -> (String, tempfile::TempDir) {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("config.json");
    let mut file = File::create(&file_path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    (file_path.to_str().unwrap().to_string(), dir) // return dir to keep it alive
  }

  #[test]
  fn test_create_command_creates_files_and_folders() {
    // Hold config temp dir alive
    let (config_path, _config_dir) = write_temp_config(&sample_config_json());

    // Hold output temp dir alive
    let outdir = tempdir().unwrap();
    let outdir_path = outdir.path().to_str().unwrap().to_string();

    let mut raw_args = HashMap::new();
    raw_args.insert("--variant".to_string(), "basic".to_string());
    raw_args.insert("--outdir".to_string(), outdir_path.clone());
    raw_args.insert("--config".to_string(), config_path.clone());
    raw_args.insert(
      "--args".to_string(),
      r#"{"name": "Ahmed", "greet": "Hello world"}"#.to_string(),
    );

    let create = Create::from_args(&raw_args);
    create_command(&create);

    // ✅ Assert expected files
    let hello_path = outdir.path().join("hello.txt");
    let hello_content = fs::read_to_string(hello_path)
      .unwrap()
      .to_string()
      .replace("{{", "")
      .replace("}}", "");
    assert_eq!(hello_content, "Hello, Ahmed!");

    let main_rs_path = outdir.path().join("src").join("main.rs");
    let main_content = fs::read_to_string(main_rs_path)
      .unwrap()
      .to_string()
      .replace("{{", "")
      .replace("}}", "");
    assert_eq!(main_content, "fn main() { println!(\"Hello world\"); }");
  }
}
