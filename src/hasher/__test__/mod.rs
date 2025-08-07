#[cfg(test)]
mod tests {
  use crate::hasher::{generate_id, CHARSET};

  #[test]
  fn test_generate_id_length() {
    let id = generate_id();
    assert_eq!(id.len(), 4, "ID should be 4 characters long");
  }

  #[test]
  fn test_generate_id_charset() {
    let id = generate_id();
    for c in id.chars() {
      assert!(
        CHARSET.contains(&(c as u8)),
        "Character '{}' not in CHARSET",
        c
      );
    }
  }

  #[test]
  fn test_generate_id_uniqueness() {
    let a = generate_id();
    std::thread::sleep(std::time::Duration::from_millis(1));
    let b = generate_id();
    assert_ne!(a, b, "IDs generated at different times should be different");
  }
}
