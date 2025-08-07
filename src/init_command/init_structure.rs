use std::collections::HashMap;

use crate::parse_commands::{commands_structure::FlagHelp, get_flag_value::get_command_value};

#[derive(Debug, Clone)]
pub struct Init {
  pub name: String,
}
impl Init {
  pub fn from_args(raw_args: &HashMap<String, String>) -> Self {
    Self {
      name: get_command_value("--name", "-n", raw_args),
    }
  }
}

pub const INIT_FLAGS: &[FlagHelp] = &[FlagHelp {
  long: "--name",
  short: "-n",
  description: "Project name",
  takes_value: true,
}];
