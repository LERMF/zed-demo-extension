# Quick Reference Guide

## Installation

```bash
# Prerequisites
rustup target add wasm32-unknown-unknown

# Clone repository
git clone https://github.com/LERMF/zed-demo-extension.git
cd zed-demo-extension

# Install as dev extension in Zed
# Cmd+Shift+X -> Install Dev Extension -> Select folder
```

## Project Structure

```
.
├── extension.toml           # Extension manifest (REQUIRED)
├── Cargo.toml              # Rust config (REQUIRED for code)
├── LICENSE                 # License file (REQUIRED for publishing)
├── src/
│   └── lib.rs             # Rust code (Extension trait)
├── languages/             # Optional
│   └── demo-lang/
│       ├── config.toml
│       └── highlights.scm
└── themes/                # Optional
    └── demo-theme.json
```

## Key Configuration Fields

### extension.toml
```toml
id = "unique-id"           # Must be unique globally
name = "Display Name"      # User-friendly name
version = "0.0.1"         # Semantic versioning
schema_version = 1         # Currently always 1
authors = ["Name <email>"]
description = "..."
repository = "https://..."
```

### Cargo.toml
```toml
[lib]
crate-type = ["cdylib"]   # WebAssembly

[dependencies]
zed_extension_api = "0.1"

[profile.release]
opt-level = "z"            # Optimize size
lto = true                 # Link time optimization
strip = true               # Strip symbols
```

## Common Commands

| Command | Purpose |
|---------|----------|
| `cargo build --target wasm32-unknown-unknown` | Build WASM |
| `cargo clippy` | Lint code |
| `cargo fmt` | Format code |
| `zed --foreground` | Debug with verbose logging |
| `zed: open log` | View extension logs |
| `zed: reload extension` | Reload dev extension |

## Writing Extension Code

```rust
use zed_extension_api as zed;

pub struct MyExtension {}

impl zed::Extension for MyExtension {
    fn new() -> Self { Self {} }
    
    fn language_server_command(
        &mut self,
        language_name: &str,
        _worktree: &zed::Worktree,
    ) -> Option<zed::Command> {
        // Return LSP command
        None
    }
}

zed::register_extension!(MyExtension);
```

## Language Configuration

```toml
# languages/my-lang/config.toml
name = "My Language"
grammar = "tree-sitter-my-lang"
path_suffixes = ["ml"]

[indent]
tab_size = 4
hard_tabs = false

[[language_servers]]
language_server_id = "my-lsp"
```

## Syntax Highlighting (Tree-sitter)

```scm
; languages/my-lang/highlights.scm
[("if") ("else") ("while")] @keyword
(comment) @comment
(string_literal) @string
```

## Theme Configuration

```json
{
  "name": "My Theme",
  "type": "dark",
  "colors": {
    "background": "#1e1e1e",
    "foreground": "#d4d4d4",
    "accent": "#007acc"
  }
}
```

## Publishing Steps

1. Update version in `extension.toml` and `Cargo.toml`
2. Make repository public on GitHub
3. Fork https://github.com/zed-industries/extensions
4. Add your extension as submodule
5. Update `extensions.toml`
6. Submit PR

## Troubleshooting

| Issue | Solution |
|-------|----------|
| "Extension not loading" | Check `zed: open log` |
| WASM compilation fails | `rustup update` and ensure Rust 1.70+ |
| LSP not starting | Verify command exists in PATH |
| Slow performance | Optimize Tree-sitter queries |
| Large WASM binary | Enable LTO and strip in profile.release |

## Resources

- [Zed Docs](https://zed.dev/docs/extensions)
- [API Reference](https://docs.rs/zed_extension_api/)
- [Tree-sitter Docs](https://tree-sitter.github.io/tree-sitter/)
- [Example Extensions](https://github.com/zed-industries/extensions)

## License Options

Pick ONE for publishing:
- MIT
- Apache 2.0
- BSD 3-Clause
- GNU GPLv3

---

**Created:** 2024
**Repository:** https://github.com/LERMF/zed-demo-extension
