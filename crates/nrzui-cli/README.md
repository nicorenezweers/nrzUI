# UI CLI

A CLI tool to add **nrzui** components to your Leptos project.

## Installation

```bash
cargo install --git https://git.zweers.ch/nicorenezweers/nrzUI nrzui-cli --force
```

## Commands

```bash
nrzui starters                  # clone a starter project
nrzui init                      # set up an existing project
nrzui add                       # interactive component picker
nrzui add button card           # add components directly
nrzui add button --dry-run      # preview without installing
nrzui add button -y             # skip overwrite prompt
nrzui list                      # browse all components
nrzui search <query>            # search components by name
nrzui view <name>               # view a component's source
nrzui diff                      # compare installed vs registry
nrzui update                    # check for outdated components
nrzui mcp                       # start the MCP server (for AI editors)
nrzui mcp init --client claude  # write editor config for MCP
```

## MCP (AI Editor Integration)

Run `nrzui mcp init --client <claude|cursor|vscode|opencode>` once in your project.
Your editor will auto-connect and can then browse, search, and add components for you.

## Links

- Docs & components: [ui.zweers.ch](https://ui.zweers.ch)
- Changelog: [CHANGELOG.md](./CHANGELOG.md)

## License

MIT — see [LICENSE](./LICENSE).
