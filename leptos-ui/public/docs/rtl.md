+++
title = "RTL Support"
description = "Enable right-to-left layout support in your nrzUI project. Physical Tailwind CSS classes are automatically transformed to logical equivalents when installing components."
tags = []
is_new = false
image = "/images/thumbnails/_placeholder.webp"
image_dark = "/images/thumbnails/_placeholder-dark.webp"
+++

## RTL Support

When RTL is enabled, the CLI automatically transforms physical Tailwind CSS classes to their logical equivalents when you run `nrzui add`. This means components work correctly in both LTR and RTL layouts without manual changes.

---

## Enabling RTL

### During `nrzui init`

```bash
nrzui init --rtl
```

You can also disable it explicitly:

```bash
nrzui init --no-rtl
```

Without a flag, `nrzui init` will prompt:

```
? Enable RTL support? (y/N)
```

### Manually in `ui_config.toml`

```toml
rtl = true
```

---

## What Gets Transformed

When `rtl = true`, every component installed via `nrzui add` has its physical Tailwind classes replaced with logical equivalents:

| Physical | Logical |
|---|---|
| `ml-` | `ms-` |
| `mr-` | `me-` |
| `pl-` | `ps-` |
| `pr-` | `pe-` |
| `left-` | `start-` |
| `right-` | `end-` |
| `rounded-tl-` | `rounded-ss-` |
| `rounded-tr-` | `rounded-se-` |
| `rounded-bl-` | `rounded-es-` |
| `rounded-br-` | `rounded-ee-` |
| `rounded-l-` | `rounded-s-` |
| `rounded-r-` | `rounded-e-` |
| `border-l-` | `border-s-` |
| `border-r-` | `border-e-` |
| `text-left` | `text-start` |
| `text-right` | `text-end` |
| `float-left` | `float-start` |
| `float-right` | `float-end` |
| `clear-left` | `clear-start` |
| `clear-right` | `clear-end` |
| `origin-top-left` | `origin-top-start` |
| `origin-top-right` | `origin-top-end` |
| `origin-left` | `origin-start` |
| `origin-right` | `origin-end` |

In addition, these classes get an `rtl:` variant appended:

- `space-x-{n}` → also adds `rtl:space-x-reverse`
- `divide-x-{n}` → also adds `rtl:divide-x-reverse`
- `translate-x-{n}` → also adds `rtl:-translate-x-{n}`
- `-translate-x-{n}` → also adds `rtl:translate-x-{n}`
- `cursor-w-resize` ↔ `cursor-e-resize` (swapped)

---

## Verifying RTL is Enabled

```bash
nrzui info
```

```
  Config file   ui_config.toml
  Base color    neutral
  Base path     src/components
  RTL           yes
  ...
```
