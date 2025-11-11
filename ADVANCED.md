# Advanced Extension Configuration Examples

This file contains advanced configuration examples for Zed extensions.

## 1. Multi-Language Server Configuration

```toml
# extension.toml
[language_servers.rust]
language_server_id = "rust-analyzer"
command = "rust-analyzer"
args = []

[language_servers.python]
language_server_id = "pylsp"
command = "pylsp"
args = []

[language_servers.go]
language_server_id = "gopls"
command = "gopls"
args = ["serve"]
```

## 2. Formatter and Linter Configuration

```toml
# languages/my-lang/config.toml

[[formatters]]
language = "my-language"
command = "my-formatter"
args = ["--format"]

[[linters]]
language = "my-language"
command = "my-linter"
args = ["--check"]
```

## 3. Custom Snippets

```toml
[[snippets]]
label = "function"
description = "Function template"
language = "my-language"
prefix = "fn"
body = "function ${1:name}() {\n\t$0\n}"
```

## 4. Semantic Highlighting

```json
{
  "semantic_tokens": {
    "function": { "color": "#dcdcaa" },
    "variable": { "color": "#9cdcfe" },
    "keyword": { "color": "#569cd6" },
    "comment": { "color": "#6a9955" },
    "string": { "color": "#ce9178" },
    "number": { "color": "#b5cea8" }
  }
}
```

## 5. Debugger Integration

```rust
// src/lib.rs
impl zed::Extension for MyExtension {
    fn debugger_command(
        &mut self,
        language_name: &str,
        _worktree: &zed::Worktree,
    ) -> Option<zed::Command> {
        match language_name {
            "rust" => Some(zed::Command {
                command: "lldb".to_string(),
                args: vec![],
                env: Default::default(),
            }),
            _ => None,
        }
    }
}
```

## 6. Theme Variants

```json
{
  "name": "My Theme",
  "variants": [
    {
      "name": "light",
      "colors": {
        "background": "#ffffff",
        "foreground": "#000000"
      }
    },
    {
      "name": "dark",
      "colors": {
        "background": "#1e1e1e",
        "foreground": "#d4d4d4"
      }
    }
  ]
}
```

## 7. File Watchers

```toml
[[file_watchers]]
glob = "**/*.toml"
events = ["create", "modify"]
```

## 8. Context Menu Items

```rust
// Custom context menu handling
impl zed::Extension for MyExtension {
    fn context_menu_items(
        &mut self,
        context: &zed::ContextMenu,
    ) -> Vec<zed::MenuItem> {
        vec![
            zed::MenuItem {
                label: "Custom Action".to_string(),
                action: "my-action".to_string(),
            },
        ]
    }
}
```

## Best Practices

### Performance
- ✅ Use Tree-sitter for fast parsing
- ✅ Implement incremental parsing when possible
- ✅ Cache language server responses
- ✅ Minimize WASM size with `strip = true` in Cargo.toml

### Compatibility
- ✅ Test with multiple Zed versions
- ✅ Use stable zed_extension_api versions
- ✅ Document breaking changes
- ✅ Provide migration guides

### User Experience
- ✅ Clear error messages
- ✅ Graceful fallbacks
- ✅ Sensible defaults
- ✅ Customizable settings

### Code Quality
- ✅ Use `clippy` for linting
- ✅ Format with `rustfmt`
- ✅ Write tests
- ✅ Document public APIs

## Testing Framework

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension_creation() {
        let extension = MyExtension::new();
        assert!(true);
    }

    #[test]
    fn test_language_server_command() {
        let mut extension = MyExtension::new();
        let cmd = extension.language_server_command("rust", &mock_worktree());
        assert!(cmd.is_some());
    }
}
```

## Common Issues and Solutions

### Issue: Extension not reloading
**Solution:** Check for compilation errors in `zed: open log`

### Issue: Language server crashes
**Solution:** Verify LSP implementation and check server logs

### Issue: Slow syntax highlighting
**Solution:** Optimize Tree-sitter queries, use incremental parsing

### Issue: Large WASM binary
**Solution:** Enable LTO and strip in Cargo.toml profile.release
