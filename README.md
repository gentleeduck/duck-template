<p align="center">
  <img src="./public/logo-dark.svg" alt="duck-template" width="120"/>
</p>

<h1 align="center">duck-template</h1>

<p align="center">
  Fast, JSON-driven Rust scaffolder for projects, files, and variants.
</p>

<p align="center">
  <a href="./LICENSE">MIT</a> -
  <a href="./CHANGELOG.md">Changelog</a> -
  <a href="./CONTRIBUTING.md">Contributing</a> -
  <a href="https://crates.io/crates/duck-template">crates.io</a> -
  <a href="https://docs.rs/duck-template">docs.rs</a>
</p>

<p align="center">
  <a href="https://crates.io/crates/duck-template"><img src="https://img.shields.io/crates/v/duck-template.svg" alt="crates.io"/></a>
  <a href="https://docs.rs/duck-template"><img src="https://docs.rs/duck-template/badge.svg" alt="docs.rs"/></a>
  <a href="./LICENSE"><img src="https://img.shields.io/crates/l/duck-template.svg" alt="MIT"/></a>
</p>

---

## Install

```sh
cargo install duck-template
```

## Quick start

```sh
duck-template init
duck-template create my-app --variant cli
duck-template create-variant web
```

The generator reads a template descriptor (local file or remote URL)
and emits the project tree from JSON. Flags are injected dynamically
into template placeholders.

## Commands

| Command | What |
| --- | --- |
| `duck-template init` | interactive setup |
| `duck-template create <name> [--variant]` | generate from current template |
| `duck-template create-variant <name>` | add a variant under the template |

## Schema

A template is a JSON file with this shape:

```json
{
  "name": "my-template",
  "variants": {
    "cli":  { "files": [ ] },
    "web":  { "files": [ ] }
  },
  "flags": { "name": "string", "rust_version": "string" }
}
```

See [`public/schema.json`](public/schema.json) for the full schema.

## Build

```sh
git clone https://github.com/gentleeduck/duck-template
cd duck-template
cargo build --release
./target/release/duck-template --help
```

## Docs

- [crates.io](https://crates.io/crates/duck-template)
- [docs.rs](https://docs.rs/duck-template)
- [duck-ui website](https://github.com/gentleeduck/duck-ui) - cross-linked

## Contributing

PR checklist + style notes in [`CONTRIBUTING.md`](CONTRIBUTING.md).
Security: [`SECURITY.md`](SECURITY.md). Behaviour: [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).

## License

MIT. See [`LICENSE`](LICENSE).
