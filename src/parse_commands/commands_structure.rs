use crate::{
  create_command::create_structure::CREATE_FLAGS,
  create_variant_command::create_variant_structure::CREATE_VARIANT_FLAGS,
  init_command::init_structure::INIT_FLAGS,
};

pub struct FlagHelp {
  pub long: &'static str,
  pub short: &'static str,
  pub description: &'static str,
  pub takes_value: bool,
}

pub struct CommandHelp {
  pub command: &'static str,
  pub description: &'static str,
  pub flags: &'static [FlagHelp],
}

pub const GLOBAL_FLAGS: &[FlagHelp] = &[
  FlagHelp {
    long: "--help",
    short: "-h",
    description: "Print help",
    takes_value: false,
  },
  FlagHelp {
    long: "--version",
    short: "-v",
    description: "Print version",
    takes_value: false,
  },
];

pub const ALL_COMMANDS: &[CommandHelp] = &[
  CommandHelp {
    command: "init",
    description: "Initializes a new project",
    flags: INIT_FLAGS,
  },
  CommandHelp {
    command: "create",
    description: "Creates a new template",
    flags: CREATE_FLAGS,
  },
  CommandHelp {
    command: "create-variant",
    description: "Creates a new template variant",
    flags: CREATE_VARIANT_FLAGS,
  },
];
