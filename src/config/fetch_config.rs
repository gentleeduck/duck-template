use reqwest::{Error, blocking::get};

pub fn curl_if_valid_url(input: &str) -> Result<String, Error> {
  let response = get(input)?;
  let body = response.text()?;
  Ok(body)
}
