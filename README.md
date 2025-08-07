# 🦆 duck-template

> A fast, customizable CLI tool for scaffolding project templates from structured JSON configurations.

`duck-template` lets you define reusable project templates in JSON and generate boilerplate code with ease using a simple, flexible command-line interface.

---

## ✨ Features

* 🧱 **Template-based project scaffolding**
* ⚙️ **Flexible `init` and `create` commands**
* 🧠 **Unified flag parsing via `get_flag_value`**
* 📦 **JSON-driven configuration system**
* 🪶 **Written in Rust — lightweight and blazing fast**
* 💬 **Helpful logs and friendly error messages**

---

## 📦 Installation

```bash
cargo install duck-template
```

---

## 🚀 Usage

### 🔧 Initialize a new project

```bash
duck-template init --name my-app
```

This will:

* Create a new folder using your template
* Inject values like `--name` into templated files
* Use your local or default `duck-template.json`

---

### 🏗️ Create a variant

```bash
duck-template create --variant api
```

This will:

* Pick a variant defined in your JSON config
* Generate all related files and folders into the specified `outdir`

---

## 🧩 Configuration

The CLI expects a `duck-template.json` config file like:

```json
{
  "$schema": "https://zpgqhogoevbgpxustvmo.supabase.co/storage/v1/object/public/json/duck-template-schema.json",
  "name": "my-template",
  "version": "0.1.0",
  "description": "A customizable web app starter template.",
  "outdir": "./output",
  "variants": [
    {
      "name": "api",
      "description": "Basic Express API setup",
      "files": [
        { "path": "src/index.ts", "content": "console.log('API running');" },
        { "path": "tsconfig.json", "template": "tsconfig-template.json" }
      ]
    }
  ]
}
```

Key fields:

* `variants`: List of available build targets
* `outdir`: Target directory for output
* `template`: File injection with variable placeholders (`{{name}}`)

---

## 🧪 Example

```bash
duck-template init --name wiseman
```

Given this config:

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

You’ll get:

```
output/
└── src/
    └── main.ts   // console.log("wiseman is wise");
```

---

## 🛠️ Commands

| Command     | Description                             |
| ----------- | --------------------------------------- |
| `init`      | Initializes a new template project      |
| `create`    | Generates output from a defined variant |
| `--help`    | Displays help information               |
| `--version` | Prints the current version              |

---

## 🔐 License

Licensed under either:

* MIT License
* Apache License, Version 2.0

---

## 💬 Feedback / Contributions

Feel free to open issues, request features, or submit a pull request.

---

> 🦆 **duck-template** — because smart devs don’t start from scratch.
