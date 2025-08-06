# 🦆 duck-template

> A fast, customizable CLI tool for scaffolding project templates from structured JSON configurations.

`duck-template` lets you define reusable project templates in JSON and generate boilerplate code with ease using a simple, flexible command-line interface.

---

## ✨ Features

- 🧱 **Template-based project scaffolding**
- ⚙️ **Powerful `init` and `create` commands**
- 🧠 **Shared flag parsing with `get_flag_value` abstraction**
- 📦 **JSON-based configuration support**
- 🪶 **Written in Rust — lightweight and blazing fast**
- 💬 **Helpful logging and error messages**


---

## 📦 Installation

```bash
cargo install duck-template
````

---

## 🚀 Usage

### Initialize a new project

```bash
duck-template init --name my-app
```

This will:

* Read your `duck-template.json` config
* Create a new output folder with the structure defined in the template
* Inject any CLI-provided values (e.g. `--name`) into the template

### Create a variant from your config

```bash
duck-template create --variant api
```

This will:

* Select a template variant defined in your config file
* Generate the corresponding output structure

---

## 🧩 Configuration

The CLI expects a JSON config file like this:

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

You can define:

* `variants`: groups of files or folders
* `outdir`: where the output should be created
* `template`: reuse external files and inject dynamic variables

---

## 🧪 Example

```bash
duck-template init --name wiseman
```

```json
// duck-template.json
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

Results in:

```
output/
└── src/
    └── main.ts // => console.log("wiseman is wise");
```

---

## 🛠 Commands

| Command   | Description                             |
| --------- | --------------------------------------- |
| `init`    | Initializes a new template project      |
| `create`  | Generates output from a defined variant |
| `help`    | Shows the help menu                     |
| `version` | Prints the CLI version                  |

---

## 🔐 License

Licensed under either of:

* MIT License
* Apache License, Version 2.0

---

## 💬 Feedback / Contributions

Feel free to open issues, request features, or contribute via pull requests.

---

> 🦆 **duck-template** — because smart devs don't start from scratch every time.
