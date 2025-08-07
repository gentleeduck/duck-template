mod __test__;
use std::{error::Error, path::Path};

use crate::{
  config::{config_structure::Variant, read_config, serialize_config, write_config},
  hasher::generate_id,
  logger::{LogLevel, log},
  template::parse_source,
};

pub mod create_variant_structure;

pub fn create_variant_command(
  create_variant: &create_variant_structure::CreateVariant,
) -> Result<String, Box<dyn Error>> {
  log(
    LogLevel::Info,
    &format!("🛠 Creating variant... {:?}", create_variant.name),
  );

  let str_buf = read_config(&create_variant.config);
  let mut config = serialize_config(&str_buf);

  let name = if create_variant.name.is_empty() {
    format!("{}-{}", config.name, generate_id())
  } else {
    create_variant.name.clone()
  };

  if create_variant.source.is_empty() {
    return Err("Missing --source/-s argument.".into());
  }

  let root = Path::new(&create_variant.source);
  println!("🛠 Parsing source...");
  let src_structure = parse_source(root, root);

  let variant = Variant {
    name: name.clone(),
    description: create_variant.description.clone(),
    src: vec![src_structure],
  };

  config.variants.push(variant);
  write_config(&create_variant.config, &config);

  log(LogLevel::Success, &format!("🛠 Created variant {}...", name));

  Ok(name)
}
