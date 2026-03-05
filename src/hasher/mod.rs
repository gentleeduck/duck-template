mod __test__;
use std::time::{SystemTime, UNIX_EPOCH};

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const ID_LEN: usize = 4;

pub fn generate_id() -> String {
  let millis = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_millis();

  let mut id = String::new();
  let mut value = millis;

  for _ in 0..ID_LEN {
    let idx = (value % CHARSET.len() as u128) as usize;
    id.push(CHARSET[idx] as char);
    value /= CHARSET.len() as u128;
  }

  id
}
