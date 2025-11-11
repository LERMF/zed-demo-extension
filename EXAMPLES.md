# Código avançado e exemplos de extensões Zed

## 1. Extension Completa com Múltiplos Language Servers

```rust
use zed_extension_api as zed;
use std::fs;
use std::path::Path;

pub struct AdvancedExtension {
    language_servers: Vec<String>,
}

impl zed::Extension for AdvancedExtension {
    fn new() -> Self {
        Self {
            language_servers: vec![
                "rust-analyzer".to_string(),
                "pylsp".to_string(),
                "gopls".to_string(),
            ],
        }
    }

    fn language_server_command(
        &mut self,
        language_name: &str,
        worktree: &zed::Worktree,
    ) -> Option<zed::Command> {
        match language_name {
            "rust" => Some(zed::Command {
                command: "rust-analyzer".to_string(),
                args: vec![],
                env: std::collections::HashMap::new(),
            }),
            "python" => Some(zed::Command {
                command: "pylsp".to_string(),
                args: vec![],
                env: std::collections::HashMap::new(),
            }),
            "go" => Some(zed::Command {
                command: "gopls".to_string(),
                args: vec!["serve".to_string()],
                env: std::collections::HashMap::new(),
            }),
            _ => None,
        }
    }

    fn indexing_settings(
        &mut self,
        _worktree: &zed::Worktree,
    ) -> Option<zed::IndexingSettings> {
        Some(zed::IndexingSettings { enabled: true })
    }
}

zed::register_extension!(AdvancedExtension);
```

## 2. Extension com Variáveis de Ambiente

```rust
use std::collections::HashMap;

impl zed::Extension for MyExtension {
    fn language_server_command(
        &mut self,
        language_name: &str,
        _worktree: &zed::Worktree,
    ) -> Option<zed::Command> {
        let mut env = HashMap::new();
        env.insert("RUST_LOG".to_string(), "debug".to_string());
        env.insert("RUST_BACKTRACE".to_string(), "1".to_string());

        match language_name {
            "rust" => Some(zed::Command {
                command: "rust-analyzer".to_string(),
                args: vec![],
                env,
            }),
            _ => None,
        }
    }
}
```

## 3. Extension com Configuração Dinâmica

```rust
use std::fs;
use std::path::Path;

pub struct ConfigurableExtension {
    config: serde_json::Value,
}

impl ConfigurableExtension {
    fn load_config(&mut self, worktree_root: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = worktree_root.join(".zed-config.json");
        if config_path.exists() {
            let content = fs::read_to_string(config_path)?;
            self.config = serde_json::from_str(&content)?;
        }
        Ok(())
    }

    fn get_lsp_command(&self, language: &str) -> Option<String> {
        self.config[language]["lsp_command"]
            .as_str()
            .map(|s| s.to_string())
    }
}
```

## 4. Tree-sitter Queries Avançadas

```scm
; highlights.scm - Exemplo avançado

; Keywords com contexto
[("if") ("else") ("while") ("for") ("return") ("fn")] @keyword
[("let") ("const") ("static")] @keyword.storage

; Functions com ñame capture
(function_declaration
  name: (identifier) @function)

(function_call
  function: (identifier) @function.call)

; Tipos
(type_annotation
  type: (type_identifier) @type)

; Comments com hierarquia
(line_comment) @comment
(block_comment) @comment
(doc_comment) @comment.documentation

; Strings e escapes
(string_literal) @string
(string_literal
  escape_sequence: (escape_sequence) @escape)

; Numbers com tipos
(integer_literal) @number
(float_literal) @number.float
(hex_literal) @number.hex

; Operadores
["=" "+" "-" "*" "/" "%" "==" "!=" "<" ">" "<="
 ">="] @operator

; Propriedades de pontuação
["(" ")" "[" "]" "{" "}" ";" ":" "," "."] @punctuation

; Scopes
(block) @scope
(function_body) @scope
```

## 5. Tema Avançado com Variações

```json
{
  "name": "Advanced Theme",
  "author": "Your Name",
  "type": "dark",
  "variants": [
    {
      "name": "Default",
      "colors": {
        "background": "#1e1e1e",
        "foreground": "#d4d4d4",
        "accent": "#007acc"
      },
      "semantic_tokens": {
        "function": { "color": "#dcdcaa", "italic": true },
        "variable": { "color": "#9cdcfe" },
        "keyword": { "color": "#569cd6", "bold": true },
        "type": { "color": "#4ec9b0" },
        "comment": { "color": "#6a9955", "italic": true },
        "string": { "color": "#ce9178" },
        "number": { "color": "#b5cea8" }
      }
    },
    {
      "name": "Light",
      "colors": {
        "background": "#ffffff",
        "foreground": "#000000",
        "accent": "#0066cc"
      },
      "semantic_tokens": {
        "function": { "color": "#795e26" },
        "variable": { "color": "#001080" },
        "keyword": { "color": "#0000ff" },
        "type": { "color": "#267f99" },
        "comment": { "color": "#008000" },
        "string": { "color": "#a31515" },
        "number": { "color": "#098658" }
      }
    }
  ]
}
```

## 6. Language Configuration Completa

```toml
# languages/my-lang/config.toml

name = "My Language"
grammar = "tree-sitter-my-lang"
path_suffixes = ["ml", "mls"]

# Language Server
[[language_servers]]
language_server_id = "my-lang-lsp"

[language_servers.initialization_options]
maxNumberOfProblems = 100
targetVersion = "latest"

# Indentação
[indent]
tab_size = 2
hard_tabs = false

# Formatador
[[formatters]]
command = "my-lang-format"
args = []

# Linter
[[linters]]
command = "my-lang-lint"
args = []

# Snippets
[[snippets]]
prefix = "fn"
label = "Function"
description = "Define uma função"
body = "function ${1:name}(${2:args}) {\n\t${3:body}\n}"

[[snippets]]
prefix = "if"
label = "If Statement"
body = "if ${1:condition} {\n\t${2:body}\n}"

# Delimitadores
[bracket_pairs]
[[bracket_pairs]]
open = "{"
close = "}"
close_newline = true
open_newline = true

[[bracket_pairs]]
open = "["
close = "]"

[[bracket_pairs]]
open = "("
close = ")"
```

## 7. Debugger Integration (Exemplo DAP)

```rust
// Exemplo de integração com Debug Adapter Protocol

impl zed::Extension for DebugAwareExtension {
    fn debugger_command(
        &mut self,
        language_name: &str,
        _worktree: &zed::Worktree,
    ) -> Option<zed::Command> {
        match language_name {
            "rust" => Some(zed::Command {
                command: "lldb-mi".to_string(),
                args: vec![],
                env: Default::default(),
            }),
            "python" => Some(zed::Command {
                command: "debugpy".to_string(),
                args: vec!["--listen", "5678"],
                env: Default::default(),
            }),
            _ => None,
        }
    }
}
```

## 8. Performance Optimization

```rust
// Cache para evitar cálculos repetidos
use std::collections::HashMap;

pub struct CachedExtension {
    cache: HashMap<String, String>,
}

impl CachedExtension {
    fn get_or_compute(&mut self, key: &str, compute: impl Fn() -> String) -> String {
        if let Some(cached) = self.cache.get(key) {
            return cached.clone();
        }
        let value = compute();
        self.cache.insert(key.to_string(), value.clone());
        value
    }
}
```

## 9. Testes Unitários

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension_new() {
        let ext = MyExtension::new();
        // Assertions
    }

    #[test]
    fn test_language_server_rust() {
        let mut ext = MyExtension::new();
        let cmd = ext.language_server_command("rust", &mock_worktree());
        assert!(cmd.is_some());
        assert_eq!(cmd.unwrap().command, "rust-analyzer");
    }

    #[test]
    fn test_language_server_unknown() {
        let mut ext = MyExtension::new();
        let cmd = ext.language_server_command("unknown", &mock_worktree());
        assert!(cmd.is_none());
    }

    // Mock para testes
    fn mock_worktree() -> zed::Worktree {
        // Implementar mock apropriado
        todo!()
    }
}
```

## 10. CI/CD com GitHub Actions

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: wasm32-unknown-unknown
      - name: Lint
        run: cargo clippy --target wasm32-unknown-unknown
      - name: Build
        run: cargo build --target wasm32-unknown-unknown --release
      - name: Test
        run: cargo test
```

---

## Dicas de Performance

1. **Minimize WASM Size**
   ```toml
   [profile.release]
   opt-level = "z"  # Optimize for size
   lto = true
   strip = true
   ```

2. **Use Caching**
   - Cache Language Server responses
   - Cache parsing results
   - Evite computations repetidas

3. **Optimize Queries**
   - Mantenha Tree-sitter queries simples
   - Evite queries muito complexas
   - Teste performance com arquivos grandes

4. **Lazy Loading**
   - Carregue resources sob demanda
   - Inicialize apenas quando necessário
   - Implemente cache com TTL

---

## Debugging Tips

```bash
# Ver output da extensão
zed --foreground

# Abrir logs
zed: open log

# Recarregar extensão
zed: reload extension

# Debug WASM
zed: debug extension
```

---

Vejam os arquivos mencionados nos exemplos para implementação completa!
