use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Create {
  pub variant: String,
  pub outdir: String,
  pub args: HashMap<String, String>,
  pub config: String,
}
