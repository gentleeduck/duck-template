#[cfg(test)]
mod tests {
  use crate::config::get_config;

  use httpmock::Method::GET;
  use httpmock::MockServer;
  use std::io::Write;
  use tempfile::NamedTempFile;

  fn sample_config_json() -> String {
    r#"
        {
            "$schema": "https://example.com/schema.json",
            "name": "test-template",
            "version": "1.0.0",
            "description": "A test config",
            "outdir": "./out",
            "variants": [
                {
                    "name": "default",
                    "description": "Default variant",
                    "src": [
                        {
                            "type": "file",
                            "path": "main.rs",
                            "content": "fn main() {}"
                        }
                    ]
                }
            ]
        }
        "#
    .to_string()
  }

  #[test]
  fn test_get_config_from_file() {
    let mut temp = NamedTempFile::new().unwrap();
    write!(temp, "{}", sample_config_json()).unwrap();
    let path = temp.path().to_str().unwrap().to_string();

    let config = get_config(&path);

    assert_eq!(config.name, "test-template");
    assert_eq!(config.variants.len(), 1);
    assert_eq!(config.variants[0].name, "default");
  }

  #[test]
  fn test_get_config_from_url() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
      when.method(GET).path("/config.json");
      then.status(200).body(sample_config_json());
    });

    let url = format!("{}/config.json", &server.base_url());
    let config = get_config(&url);

    mock.assert(); // Ensure it was actually called
    assert_eq!(config.name, "test-template");
  }
}
