# Configuration Summary

## extension.toml ✅

**Localização:** `extension.toml` (raiz do projeto)

**Conteúdo:**
```toml
id = "demo-extension"
name = "Demo Extension"
version = "0.0.1"
schema_version = 1
authors = ["LERMF <user@example.com>"]
description = "A comprehensive demo extension for Zed editor showcasing best practices"
repository = "https://github.com/LERMF/zed-demo-extension"
```

**Propósito:** Define metadados e configuração básica da extensão.

---

## Estrutura de Projeto

```
zed-demo-extension/
├── extension.toml              ✅ Manifesto da extensão
├── Cargo.toml                  ✅ Configuração Rust/WebAssembly
├── LICENSE                     ✅ Licença MIT
├── .gitignore                  ✅ Ignorar arquivos desnecessários
├── README.md                   ✅ Documentação principal
├── DEVELOPMENT.md              ✅ Guia de desenvolvimento
├── ADVANCED.md                 ✅ Configuração avançada
├── src/
│   └── lib.rs                  ✅ Código principal (Extension trait)
├── languages/
│   └── demo-lang/
│       ├── config.toml         ✅ Configuração da linguagem
│       └── highlights.scm      ✅ Tree-sitter highlighting
└── themes/
    └── demo-theme.json         ✅ Tema customizado
```

---

## Como Usar

### Passo 1: Clonar o Repositório

```bash
git clone https://github.com/LERMF/zed-demo-extension.git
cd zed-demo-extension
```

### Passo 2: Instalar como Dev Extension

**Opção A - Via Interface Zed:**
1. Abra Zed
2. Pressione `Cmd+Shift+X` (macOS) ou `Ctrl+Shift+X` (Linux/Windows)
3. Clique em "Install Dev Extension"
4. Selecione a pasta do projeto

**Opção B - Via Command Line:**
```bash
zed --install-dev-extension /caminho/para/zed-demo-extension
```

### Passo 3: Desenvolvimento

1. Faça mudanças em `src/lib.rs`
2. Zed automaticamente recompila e recarrega
3. Observe logs em `zed: open log`

### Passo 4: Debug

```bash
# Terminal com verbose logging
zed --foreground
```

---

## Configurações Principais

### 1. extension.toml

| Campo | Descrição | Obrigatório |
|-------|-----------|-------------|
| `id` | Identificador único | ✅ |
| `name` | Nome legível | ✅ |
| `version` | Versão semântica | ✅ |
| `schema_version` | Versão de schema | ✅ |
| `authors` | Autores | ✅ |
| `description` | Descrição | ⚠️ Recomendado |
| `repository` | URL do repositório | ⚠️ Recomendado |

### 2. Cargo.toml

**Configuração crítica:**
```toml
[lib]
crate-type = ["cdylib"]  # Obrigatório para WASM

[dependencies]
zed_extension_api = "0.1"  # Use versão compatível

[profile.release]
opt-level = "z"  # Otimizar para tamanho
lto = true        # Link Time Optimization
strip = true      # Remove símbolos desnecessários
```

### 3. Language Configuration

**languages/demo-lang/config.toml:**
```toml
name = "Demo Language"
grammar = "tree-sitter-demo"
path_suffixes = ["demo", "dml"]

[indent]
tab_size = 2
hard_tabs = false
```

### 4. Syntax Highlighting

**languages/demo-lang/highlights.scm:**
```scm
; Tree-sitter queries
[("if") ("else") ("while")] @keyword
(comment) @comment
(string_literal) @string
```

### 5. Temas

**themes/demo-theme.json:**
```json
{
  "name": "Demo Theme",
  "type": "dark",
  "colors": {
    "background": "#1e1e1e",
    "foreground": "#d4d4d4"
  }
}
```

---

## SOTA (State of the Art) - Melhores Práticas 2024/2025

### ✅ Implementar

1. **Language Server Protocol (LSP)**
   - Suporte para IntelliSense
   - Diagnostics em tempo real
   - Code completion
   - Refactoring

2. **Tree-sitter para Highlighting**
   - Performance otimizada
   - Parsing incremental
   - Suporte a múltiplas linguagens

3. **WebAssembly (WASM)**
   - Compilação com `wasm32-unknown-unknown`
   - Otimização de tamanho
   - Segurança via sandbox

4. **Configuração Declarativa**
   - TOML para configuração
   - JSON para temas
   - Fácil manutenção

5. **Semantic Highlighting**
   - Cores baseadas em tipo
   - Melhor legibilidade
   - Suporte a temas

### ❌ Evitar

- Código não compilável
- Dependências pesadas
- Hardcoding de caminhos
- Sem documentação
- Binários muito grandes

---

## Recursos de Aprendizado

### Documentação Oficial
- [Zed Extensions](https://zed.dev/docs/extensions)
- [Zed Extension API](https://docs.rs/zed_extension_api/)
- [Tree-sitter](https://tree-sitter.github.io/tree-sitter/)

### Exemplos
- [Zed Industries Extensions](https://github.com/zed-industries/extensions)
- [Language Extensions](https://github.com/zed-industries/extensions/tree/main/extensions)
- [Theme Examples](https://github.com/zed-industries/extensions/tree/main/themes)

### Comunidade
- [Zed Discord](https://discord.gg/zed)
- [GitHub Discussions](https://github.com/zed-industries/zed/discussions)

---

## Próximos Passos

### Para Iniciantes
1. ✅ Instalar como dev extension
2. ✅ Explorar arquivos de exemplo
3. ✅ Fazer pequenas mudanças
4. ✅ Aprender Tree-sitter

### Para Intermediários
1. ✅ Adicionar language server
2. ✅ Criar tema customizado
3. ✅ Implementar snippets
4. ✅ Adicionar formatters

### Para Avançados
1. ✅ DAP (Debug Adapter Protocol)
2. ✅ MCP (Model Context Protocol)
3. ✅ Slash commands
4. ✅ Publishing

---

## Troubleshooting

| Problema | Solução |
|----------|--------|
| Extension não carrega | Verificar `zed: open log` |
| WASM não compila | Atualizar Rust: `rustup update` |
| Language server falha | Verificar PATH e args |
| Highlighting incorreto | Revisar Tree-sitter queries |
| Performance lenta | Otimizar queries, usar caching |

---

## Status do Projeto ✅

- [x] extension.toml configurado
- [x] Cargo.toml para WASM
- [x] Extension trait implementado
- [x] Language support exemplo
- [x] Tema customizado
- [x] Highlighting com Tree-sitter
- [x] Documentação completa
- [x] Guias de desenvolvimento
- [x] Repositório privado no GitHub
- [x] Pronto para dev extension

---

## Licença

MIT - Veja LICENSE para detalhes

## Autor

LERMF - github.com/LERMF
