use reqwest::blocking::get;
use reqwest::Error;

pub fn curl_if_valid_url(input: &str) -> Result<String, Error> {
  let response = get(input)?;
  let body = response.text()?;
  Ok(body)
}
