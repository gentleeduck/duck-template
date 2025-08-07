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

pub const CREATE_FLAGS: &[FlagHelp] = &[
  FlagHelp {
    long: "--variant",
    short: "-v",
    description: "Template variant",
    takes_value: true,
  },
  FlagHelp {
    long: "--outdir",
    short: "-d",
    description: "Output directory",
    takes_value: true,
  },
  FlagHelp {
    long: "--config",
    short: "-c",
    description: "Path to config file",
    takes_value: true,
  },
  FlagHelp {
    long: "--args",
    short: "-a",
    description: "Extra arguments (key=value)",
    takes_value: true,
  },
];

pub const INIT_FLAGS: &[FlagHelp] = &[FlagHelp {
  long: "--name",
  short: "-n",
  description: "Project name",
  takes_value: true,
}];

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
];
