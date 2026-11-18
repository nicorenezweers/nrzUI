+++
title = "CLI"
description = "Learn how to use the nrzUI CLI to install components, initialize projects, and manage your component library efficiently."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
order = 3
+++


## Installation

```bash
cargo install --git https://git.zweers.ch/nicorenezweers/nrzUI nrzui-cli --force
```


## Commands

### Starters (optional, quick start)

If you want to start very easily with all setup for you, run this:

```bash
nrzui starters # Optional, quick start
```


### Init (existing projects)

If you want add components to an existing project, run this:

```bash
nrzui init
```

This command will setup everything for you to then add components easily.


When you run `nrzui init`, this is what you get: 

<StaticDocsInstallationCliTreeView />


### Add

For adding new components, you just need to run this:

```bash
nrzui add button
# nrzui add demo_card demo_button
# └──> Works with any number of components
```

## Getting Started

The nrzUI CLI provides a **streamlined workflow** for building component libraries in Rust. Whether you're starting a new project or adding components to an existing codebase, the CLI handles all the complexity for you.

**Recommended workflow:**

1. **Quick Start:** Use `nrzui starters` if you want a complete project setup with example components
2. **Existing Projects:** Use `nrzui init` to configure your current project for nrzUI components  
3. **Add Components:** Use `nrzui add [component_name]` to install individual components as needed

_The CLI automatically manages dependencies, imports, and file structure, so you can focus on building your application instead of configuration._