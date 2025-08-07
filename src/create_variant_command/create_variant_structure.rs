use std::collections::HashMap;

use serde::Serialize;

use crate::parse_commands::{commands_structure::FlagHelp, get_flag_value::get_command_value};

#[derive(Debug, Clone, Serialize)]
pub struct CreateVariant {
  pub name: String,
  pub source: String,
  pub config: String,
  pub description: String,
  pub ignore: Vec<String>,
}

impl CreateVariant {
  pub fn from_args(raw_args: &HashMap<String, String>) -> Self {
    Self {
      name: get_command_value("--name", "-n", raw_args),
      source: get_command_value("--source", "-s", raw_args),
      config: get_command_value("--config", "-c", raw_args),
      description: get_command_value("--description", "-d", raw_args),
      ignore: get_command_value("--ignore", "-i", raw_args)
        .split(',')
        .map(String::from)
        .collect(),
    }
  }
}

pub const CREATE_VARIANT_FLAGS: &[FlagHelp] = &[
  FlagHelp {
    long: "--source",
    short: "-s",
    description: "Path to the source files or directory that defines the contents of the variant. This will be used as the base for the new variant.",
    takes_value: true,
  },
  FlagHelp {
    long: "--config",
    short: "-c",
    description: "Optional path to a JSON config file that defines metadata or arguments for the variant (e.g., default values, validation rules).",
    takes_value: true,
  },
  FlagHelp {
    long: "--name",
    short: "-n",
    description: "Unique name to identify the variant. This name will be used in the template selection and must not conflict with existing variants.",
    takes_value: true,
  },
  FlagHelp {
    long: "--description",
    short: "-d",
    description: "A short human-readable description for the variant, explaining its purpose or differences from other variants.",
    takes_value: true,
  },
  FlagHelp {
    long: "--ignore",
    short: "-i",
    description: "",
    takes_value: true,
  },
];
