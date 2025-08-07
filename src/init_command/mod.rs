mod __test__;
use std::collections::HashMap;

use serde_json;

use crate::{
  config::config_structure::{Config, File, Folder, Source, Variant},
  init_command::init_structure::Init,
  logger::{log, LogLevel},
};

pub mod init_structure;

pub fn init_command(init: &Init) {
  log(
    LogLevel::Info,
    &format!("🦆 Initializing project... {:?}", init.name),
  );

  let mut args = HashMap::new();
  args.insert("name".to_string(), init.name.clone());

  let template = Config {
        schema: "https://zpgqhogoevbgpxustvmo.supabase.co/storage/v1/object/public/json/duck-template-schema.json".to_string(),
        name: init.name.clone(),
        version: "0.1.0".to_string(),
        description: "Wise decisions, powered by types.".to_string(),
        outdir: Some("./duck-template-dir".to_string()),
        args: Some(args.clone()),
        variants: vec![
            Variant {
                name: "init".to_string(),
                description: "Basic TypeScript project scaffold".to_string(),
                src: vec![
                    // package.json
                    Source::File(File {
                        path: "package.json".to_string(),
                        content: r#"{ 
  "name": "{{ name }}", 
  "version": "0.1.0", 
  "main": "dist/index.js", 
  "scripts": { 
    "build": "tsc", 
    "start": "node dist/index.js" 
  }, 
  "dependencies": {}, 
  "devDependencies": { 
    "typescript": "^4.0.0" 
  } 
}"#
                        .to_string(),
                        args: Some(vec!["{{ name }}".to_string()]),
                    }),
                    // tsconfig.json
                    Source::File(File {
                        path: "tsconfig.json".to_string(),
                        content: r#"{
  "compilerOptions": {
    "target": "ES2019",
    "module": "CommonJS",
    "outDir": "dist",
    "rootDir": "src",
    "strict": true,
    "esModuleInterop": true
  }
}"#
                        .to_string(),
                        args: Some(vec![]),
                    }),
                    // src folder + index.ts
                    Source::Folder(Folder {
                        path: "src".to_string(),
                        children: vec![Source::File(File {
                            path: "src/index.ts".to_string(),
                            content: "console.log('👋 Welcome to {{ name }}!');".to_string(),
                            args: Some(vec!["{{ name }}".to_string()]),
                        })],
                    }),
                    // README.md
                    Source::File(File {
                        path: "README.md".to_string(),
                        content: "# {{ name }}\n\nA fresh TypeScript starter project.\n\n## Developing\n\n```bash\nnpm install\nnpm run build\nnpm start\n```"
                            .to_string(),
                        args: Some(vec!["{{ name }}".to_string()]),
                    }),
                    // .gitignore
                    Source::File(File {
                        path: ".gitignore".to_string(),
                        content: "node_modules/\ndist/".to_string(),
                        args: Some(vec![]),
                    }),
                ],
            },
        ],
    };

  let json =
    serde_json::to_string_pretty(&template).expect("Failed to serialize template config to JSON");

  // Write to file
  match std::fs::write(&format!("{}-template.json", init.name), json) {
    Ok(_) => {
      log(LogLevel::Info, "🦆 Project initialized successfully.");
    },
    Err(e) => log(
      LogLevel::Error,
      &format!("❌ Failed to write to file: {}", e),
    ),
  }
}
