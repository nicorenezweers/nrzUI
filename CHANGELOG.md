# Changelog

## 1.1.0

- Sicherheits-Updates (nrzSecurity): pulldown-cmark 0.13.6, cookie 0.18.3, url 2.5.9, rustls 0.23.46,
  rustls-webpki 0.103.16, webpki-roots 1.0.10, quinn-proto 0.11.19, quinn-udp 0.5.16, h2 0.4.16,
  webbrowser 1.2.2, serde_with 3.21.0, rand 0.8.6, quick-xml 0.41.0 in allen Lockfiles.
- rusqlite 0.32 → 0.43 (libsqlite3-sys 0.40) im Root- und `leptos-ui`-Workspace (Feature `server`/`ssr`).
- `leptos-ui/src-tauri`: tauri 2.11.1 (wry 0.57).
- `nrzui-cli` 1.1.0: rmcp 2 → 3.11 (`ServerInfo` → `ServerConfig`).
- Starter `start-dioxus-fullstack` / `start-tauri-fullstack`: sqlx 0.8 → 0.9 (`runtime-tokio` +
  `tls-rustls-ring-webpki`); rsa entfällt, libsqlite3-sys 0.40.
- Workspace: Versionsanforderungen der Pfad-Abhängigkeiten `icons`/`tw_merge`/`tw_merge_variants`
  auf 1.0 korrigiert (Lockfiles ließen sich sonst nicht aktualisieren).

## 1.0.2

- `.gitattributes`: Attribute mit explizitem Wert (`=true`), damit nrzGit sie auswertet;
  Browser-Hilfsskripte, Styles, Build-/Plattformdateien und Root-E2E-Suite vollständig erfasst.

## 1.0.1

- `.gitattributes`: einheitliche LF-Zeilenenden, Binärdateien markiert.
- Vendorte Bibliotheken (ApexCharts, Sonner/Vaul, Animate.css), generiertes Tailwind-CSS,
  E2E-Suiten und Build-/Hilfsdateien in der Sprachstatistik korrekt eingeordnet.

## 1.0.0

- Initial release.
