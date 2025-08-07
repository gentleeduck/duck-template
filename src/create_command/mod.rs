pub mod create_structure;

use std::collections::HashMap;
use std::path::Path;

use crate::config::config_structure::Source;
use crate::config::read_config::{read_config, serialize_config};
use crate::create_command::create_structure::Create;
use crate::logger::{log, LogLevel};
use crate::template::replace_args;

pub fn create_command(create: &Create) {
  println!("create: {:#?}", create);
  log(LogLevel::Info, "🛠 Creating project...");

  if create.config.is_empty() {
    log(LogLevel::Error, "🦆 No config file provided.");
    log(
      LogLevel::Info,
      "👉 Use `@duck-template init` to create a new template.",
    );
    log(
      LogLevel::Info,
      "👉 Use `@duck-template create` to create a new project.",
    );
    std::process::exit(1);
  }

  let str_buf = read_config(&create.config);
  let config = serialize_config(&str_buf);

  let outdir = if create.outdir.is_empty() {
    &config.outdir.unwrap_or(String::from("./duck-template-dir"))
  } else {
    &create.outdir
  };

  let outdir_path = Path::new(outdir);

  let mut args = config.args.unwrap_or_default();
  for (k, v) in &create.args {
    args.insert(k.clone(), v.clone()); // CLI args overwrite config
  }

  let variant = &config.variants.iter().find(|v| v.name == create.variant);

  match variant {
    Some(variant) => {
      log(
        LogLevel::Info,
        &format!("🦆 Creating variant: {}", variant.name),
      );
      render_source(&variant.src, outdir_path, &args);
    },
    None => {
      log(
        LogLevel::Error,
        &format!("❌ Variant '{}' not found", create.variant),
      );
      std::process::exit(1);
    },
  }

  log(
    LogLevel::Success,
    &format!("🎉 Project created at {}", outdir_path.display()),
  );
  log(
    LogLevel::Success,
    &format!("🚀 Enjoy quacking in `{}`", config.name),
  );
}

pub fn render_source(src: &[Source], outdir: &Path, cli_args: &HashMap<String, String>) {
  for item in src {
    match item {
      Source::File(file) => {
        let file_path = outdir.join(file.path.as_str());
        if let Some(parent) = file_path.parent() {
          std::fs::create_dir_all(parent).expect("❌ Failed to create parent folder");
        }

        let final_content = if let Some(args) = &file.args {
          replace_args(file.content.as_str(), cli_args, &args)
        } else {
          file.content.clone()
        };

        log(
          LogLevel::Info,
          &format!("👉 📄 Writing file: {}", file_path.display()),
        );

        std::fs::write(&file_path, final_content).expect("❌ Failed to write file");
      },

      Source::Folder(folder) => {
        let folder_path = outdir.join(folder.path.as_str());

        log(
          LogLevel::Info,
          &format!("👉 📂 Creating folder: {}", folder_path.display()),
        );

        std::fs::create_dir_all(&folder_path).expect("❌ Failed to create folder");

        render_source(&folder.children, outdir, cli_args);
      },
    }
  }
}
