# 🦆 duck-template

> Generate and manage project templates with ease — fast, customizable, and powered by Rust.

**`duck-template`** helps you scaffold and manage projects using structured JSON configurations and flexible CLI commands. With variant support, remote configs, and dynamic flag injection, it gives you full control over how projects and their files are created.

---

## ✨ Features

* 🧱 **Template-based project scaffolding** with JSON
* 🏗️ **Variants** to support different project layouts (e.g., `api`, `web`, `cli`)
* 🌍 **Remote or local config** support (pass a URL or file path)
* 🧠 **Unified flag parsing** with dynamic injection into templates
* 🪶 **Blazing fast** — written in Rust
* 📦 **Modular commands**: `init`, `create`, and `create-variant`

---

## 📦 Installation

```bash
cargo install duck-template
```

---

## 🚀 Quick Start

### 🔧 Initialize a New Project

```bash
duck-template init --name my-app
```

This will:

* Create a directory `my-app/`
* Inject the name into templated files
* Use your local `duck-template.json` file

---

### 🏗️ Create From a Variant

```bash
duck-template create --variant api --config ./duck-template.json
```

Or use a **remote config**:

```bash
duck-template create --variant api --config https://example.com/template.json
```

Optional extras:

```bash
duck-template create \
  --variant cli \
  --config ./template.json \
  --outdir ./output \
  --args author=Ahmed,year=2025
```

---

### ✨ Create a New Variant

```bash
duck-template create-variant \
  --source ./template-source \
  --name cli \
  --description "Command-line app setup" \
  --config ./duck-template.json
```

This will:

* Package the folder into a new variant
* Append it to the config (if valid and writable)

---

## 🧩 Configuration Format

You define templates in a `duck-template.json` file like this:

```json
{
  "$schema": "https://zpgqhogoevbgpxustvmo.supabase.co/storage/v1/object/public/json/duck-template-schema.json",
  "name": "my-template",
  "version": "1.0.0",
  "description": "Reusable project setup",
  "outdir": "./output",
  "args": {
    "author": "Anonymous",
    "license": "MIT"
  },
  "variants": [
    {
      "name": "api",
      "description": "Express API starter",
      "files": [
        { "path": "src/index.ts", "content": "console.log('API ready');" },
        { "path": "tsconfig.json", "template": "tsconfig-template.json" }
      ]
    }
  ]
}
```

---

## 🛠️ Command Reference

### 🔧 `init`

Create a new project directory.

```bash
duck-template init --name my-app
```

| Flag           | Description                                                         |
| -------------- | ------------------------------------------------------------------- |
| `-n`, `--name` | Name of the project. Used for the folder name and inside templates. |

---

### 🏗️ `create`

Generate files from a predefined variant.

```bash
duck-template create --variant api --config ./config.json
```

| Flag              | Description                                                       |
| ----------------- | ----------------------------------------------------------------- |
| `-v`, `--variant` | Name of the variant to generate                                   |
| `-d`, `--outdir`  | Directory to write output (defaults to `./`)                      |
| `-c`, `--config`  | Local or remote JSON config file                                  |
| `-a`, `--args`    | Key=value overrides for template injection (e.g., `author=Ahmed`) |

---

### 🧪 `create-variant`

Package a folder into a new variant.

```bash
duck-template create-variant \
  --source ./starter \
  --name basic \
  --description "Basic setup" \
  --config ./duck-template.json
```

| Flag                  | Description                                     |
| --------------------- | ----------------------------------------------- |
| `-s`, `--source`      | Source directory or file                        |
| `-n`, `--name`        | Unique name for the variant                     |
| `-d`, `--description` | Short explanation of what this variant is for   |
| `-c`, `--config`      | Optional path to update an existing config file |

---

## 📂 Output Example

Given:

```bash
duck-template init --name wiseman
```

And this config:

```json
{
  "name": "wiseman",
  "outdir": "./output",
  "variants": [
    {
      "name": "wiseman",
      "files": [
        { "path": "src/main.ts", "content": "console.log('{{name}} is wise');" }
      ]
    }
  ]
}
```

The result will be:

```
output/
└── src/
    └── main.ts   // console.log("wiseman is wise");
```

---

## 💬 Help & Version

```bash
duck-template --help
duck-template --version
```

Also works with subcommands like:

```bash
duck-template init --help
duck-template create-variant --help
```


## 🙌 Contributions

Pull requests, issues, and suggestions are welcome!
Feel free to fork, tweak, and share your own templates too.

---

> 🦆 **duck-template** — because smart devs don’t start from scratch.
