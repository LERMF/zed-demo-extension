# Zed Demo Extension

A comprehensive demo extension for the Zed editor showcasing best practices for Zed extension development.

## Features

- ✨ Extension trait implementation
- 🌍 Language server protocol support
- 🎨 Theme configuration
- 📝 Language definition example
- 🔍 Tree-sitter syntax highlighting

## Development Setup

### Prerequisites

- Rust installed via `rustup` (required for WebAssembly compilation)
- Zed editor installed

### Installation as Dev Extension

1. Clone this repository
2. Open Zed editor
3. Use `zed: install dev extension` command
4. Select this directory
5. The extension will be loaded for development

## File Structure

```
zed-demo-extension/
├── extension.toml          # Extension manifest
├── Cargo.toml             # Rust/WASM configuration
├── src/
│   └── lib.rs            # Main extension code
├── languages/
│   └── demo-lang/        # Example language support
│       ├── config.toml
│       └── highlights.scm
├── themes/
│   └── demo-theme.json   # Example theme
└── README.md
```

## Extension Capabilities

This extension demonstrates:

### 1. Extension Trait
Basic Extension trait implementation with language server and indexing support.

### 2. Language Support
Example configuration for a custom language with Tree-sitter highlighting.

### 3. Themes
Dark theme configuration following Zed theme specification.

## Debugging

To see debug output:

```bash
zed --foreground
```

Check `zed: open log` for additional debugging information.

## Publishing

To publish this extension:

1. Fork the [zed-industries/extensions](https://github.com/zed-industries/extensions) repository
2. Add this as a git submodule
3. Update `extensions.toml`
4. Submit a pull request

## License

MIT License - See LICENSE file for details

## Resources

- [Zed Extension Documentation](https://zed.dev/docs/extensions)
- [Tree-sitter Documentation](https://tree-sitter.github.io/tree-sitter/)
- [Language Server Protocol](https://microsoft.github.io/language-server-protocol/)
