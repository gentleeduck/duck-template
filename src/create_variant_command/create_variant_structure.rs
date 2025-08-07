use serde::Serialize;
use std::collections::HashMap;

use crate::parse_commands::{commands_structure::FlagHelp, get_flag_value::get_command_value};

#[derive(Debug, Clone, Serialize)]
pub struct CreateVariant {
  pub name: String,
  pub source: String,
  pub config: String,
  pub description: String,
}

impl CreateVariant {
  pub fn from_args(raw_args: &HashMap<String, String>) -> Self {
    Self {
      name: get_command_value("--name", "-n", raw_args),
      source: get_command_value("--source", "-s", raw_args),
      config: get_command_value("--config", "-c", raw_args),
      description: get_command_value("--description", "-d", raw_args),
    }
  }
}

pub const CREATE_VARIANT_FLAGS: &[FlagHelp] = &[
  FlagHelp {
    long: "--source",
    short: "-s",
    description: "Variant source",
    takes_value: true,
  },
  FlagHelp {
    long: "--config",
    short: "-c",
    description: "Variant config",
    takes_value: true,
  },
  FlagHelp {
    long: "--name",
    short: "-n",
    description: "Variant name",
    takes_value: true,
  },
  FlagHelp {
    long: "--description",
    short: "-n",
    description: "Variant description",
    takes_value: true,
  },
];
