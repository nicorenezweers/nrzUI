# nrzUI

Fork von rust-ui (Dioxus 0.7 + Tailwind v4), Komponenten in `app_crates/registry/src/ui`.

- Thema: dunkel ist Standard (nrz-Palette in `tailwind.css` unter `.dark`). Neue Komponenten müssen im
  dunklen Thema gut aussehen; nur Theme-Variablen (`bg-background`, `bg-card`, `text-muted-foreground`, …)
  verwenden, keine festen Farben.
- Clippy nur für berührte Crates: `cargo clippy -p <crate> --no-deps`.
- `tw_merge` und `icons` kommen aus `crates/` (path-Abhängigkeiten), nicht von crates.io.
- Leptos-Variante liegt in `leptos-ui/` (eigener Workspace, nutzt `../crates/{icons,tw_merge}`).
- Änderungen in `CHANGELOG.md` unter `## [Unreleased]` eintragen.
