# Development Guide for Zed Demo Extension

## Quick Start

### 1. Environment Setup

```bash
# Install Rust via rustup (required)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add wasm32 target
rustup target add wasm32-unknown-unknown

# Install Zed editor
# Download from https://zed.dev
```

### 2. Clone and Install Dev Extension

```bash
git clone https://github.com/LERMF/zed-demo-extension.git
cd zed-demo-extension
```

In Zed:
1. Press `Cmd+Shift+X` (macOS) or `Ctrl+Shift+X` (Linux/Windows) to open Extensions
2. Click "Install Dev Extension"
3. Select the cloned directory

### 3. Development Workflow

#### Edit and Reload
- Edit files in `src/lib.rs`
- Zed automatically compiles WASM and reloads
- Watch the log with `zed: open log`

#### Debug Output
```bash
# Terminal 1: Start Zed with foreground logging
zed --foreground

# Terminal 2: Make changes and observe output
```

## SOTA (State of the Art) Best Practices

### 1. Extension Trait Implementation

```rust
pub struct MyExtension {}

impl zed::Extension for MyExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        language_name: &str,
        worktree: &zed::Worktree,
    ) -> Option<zed::Command> {
        // Return LSP command for language
        None
    }
}

zed::register_extension!(MyExtension);
```

### 2. Language Server Protocol (LSP) Integration

```toml
[[language_servers]]
language_server_id = "my-lsp"
language = "my-language"

[language_servers.initialization_options]
# Configuration options for LSP
```

### 3. Tree-sitter for Syntax Highlighting

```scm
; highlights.scm - Tree-sitter queries
(keyword) @keyword
(comment) @comment
(string_literal) @string
(number_literal) @number
(function_declaration name: (identifier) @function)
```

### 4. Theme Configuration

```json
{
  "name": "My Theme",
  "author": "Your Name",
  "type": "dark",
  "colors": {
    "background": "#1e1e1e",
    "foreground": "#d4d4d4",
    "accent": "#007acc"
  }
}
```

## Extension Capabilities

### Available Features

1. **Language Support**
   - Define syntax highlighting with Tree-sitter
   - Configure language servers (LSP)
   - Set indent preferences
   - Add formatters and linters

2. **Themes**
   - Create color schemes
   - Support light/dark modes
   - Semantic highlighting support

3. **Debugging**
   - DAP (Debug Adapter Protocol) integration
   - Breakpoint support
   - Variable inspection

4. **Slash Commands**
   - Custom commands accessible via `/`
   - Integration with Copilot

5. **MCP Servers**
   - Model Context Protocol support
   - Enhanced AI capabilities

## Common Patterns

### Pattern 1: Language Server Integration

```rust
impl zed::Extension for MyExtension {
    fn language_server_command(
        &mut self,
        language_name: &str,
        worktree: &zed::Worktree,
    ) -> Option<zed::Command> {
        match language_name {
            "rust" => Some(zed::Command {
                command: "rust-analyzer".to_string(),
                args: vec![],
                env: Default::default(),
            }),
            "python" => Some(zed::Command {
                command: "pylsp".to_string(),
                args: vec![],
                env: Default::default(),
            }),
            _ => None,
        }
    }
}
```

### Pattern 2: Indexing Configuration

```rust
fn indexing_settings(
    &mut self,
    _worktree: &zed::Worktree,
) -> Option<zed::IndexingSettings> {
    Some(zed::IndexingSettings {
        enabled: true,
    })
}
```

## Testing

### Manual Testing

1. Create a test file with your language
2. Verify syntax highlighting works
3. Test language server features
4. Check theme colors

### Debug Mode

```bash
# Start with verbose logging
zed --foreground

# Check extension logs
zed: open log
```

## Troubleshooting

### Extension not loading
1. Check `zed: open log` for errors
2. Verify `extension.toml` syntax
3. Ensure `Cargo.toml` has correct configuration
4. Try removing and re-installing dev extension

### WASM compilation errors
1. Update Rust: `rustup update`
2. Check `zed_extension_api` version compatibility
3. Review compiler error messages

### Language server not starting
1. Verify LSP command exists in PATH
2. Check initialization options in `config.toml`
3. Review language server logs

## Publishing

### Steps

1. Ensure all files are properly formatted
2. Update version in `extension.toml` and `Cargo.toml`
3. Test thoroughly as dev extension
4. Create GitHub repository (public required)
5. Fork `zed-industries/extensions`
6. Add as submodule with HTTPS URL
7. Update `extensions.toml`
8. Submit PR

### Requirements

- ✅ Valid license (MIT, Apache 2.0, BSD 3-Clause, or GPLv3)
- ✅ Public GitHub repository
- ✅ Proper `extension.toml`
- ✅ Working WASM compilation
- ✅ Documentation

## Resources

- [Zed Documentation](https://zed.dev/docs)
- [Zed Extension API Docs](https://zed.dev/docs/extensions)
- [Tree-sitter Guide](https://tree-sitter.github.io/tree-sitter/)
- [LSP Specification](https://microsoft.github.io/language-server-protocol/)
- [Example Extensions](https://github.com/zed-industries/extensions)

## Contributing

Feel free to fork and modify this extension as a starting point for your own extensions.

## License

MIT - See LICENSE file
