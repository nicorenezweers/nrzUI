pub struct SiteConfig;

impl SiteConfig {
    pub const TITLE: &str = "nrzUI";
    pub const AUTHOR: &str = "nrz";
    pub const META_DESCRIPTION: &str = "nrzUI · Dioxus components for all nrzApps · Build once, run everywhere.";
    pub const DESCRIPTION: &str = "nrzUI is the dark-first, cross-platform component framework behind the nrzApps: a registry for Dioxus and Rust fullstack applications. Beautiful UI components built with Dioxus, Rust, and Tailwind CSS. Build your UI once and deploy to iOS, Android, Desktop, and Web.";
    pub const BASE_URL: &str = "https://ui.zweers.ch";
    /// Local dev server (dx serve). Matches Dioxus.toml's `[serve] port`.
    pub const BASE_URL_LOCALHOST: &str = "http://localhost:8080";
    pub const TWITTER_IMAGE: &str = "https://ui.zweers.ch/twitter-card.webp";
}
