use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Config {
  #[serde(rename = "$schema")]
  pub schema: String,
  pub name: String,
  pub version: String,
  pub description: String,
  pub outdir: Option<String>,
  pub variants: Vec<Variant>,
  #[serde(default)]
  pub args: Option<HashMap<String, String>>,
  #[serde(default)]
  pub ignore: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Variant {
  pub name: String,
  pub description: String,
  pub src: Vec<Source>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(tag = "type")]
pub enum Source {
  #[serde(rename = "file")]
  File(File),
  #[serde(rename = "folder")]
  Folder(Folder),
  Skip,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct File {
  pub path: String,
  pub content: String,
  #[serde(default)]
  pub args: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Folder {
  pub path: String,
  pub children: Vec<Source>,
}
