use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Init {
  pub name: String,
}
impl Init {
  pub fn from_args(args: &HashMap<String, String>) -> Self {
    println!("args: {:#?}", args);
    Self {
      name: args
        .get("--name")
        .cloned()
        .unwrap_or(args.get("-n").cloned().unwrap_or_default()),
    }
  }
}
