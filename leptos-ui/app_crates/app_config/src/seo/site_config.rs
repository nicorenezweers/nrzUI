// TODO. Create a proper impl with SiteConfig. See Shadcn.

pub struct SiteConfig;

impl SiteConfig {
    pub const TITLE: &str = "nrzUI";
    pub const AUTHOR: &str = "nrz";
    pub const META_DESCRIPTION: &str = "Leptos nrzUI Components · Build once, run Everywhere.";
    pub const DESCRIPTION: &str = "nrzUI is a cross-platform component registry for Leptos and Rust fullstack applications. Beautiful UI components built with Leptos, Rust, and Tailwind CSS. Build your UI once and deploy to iOS, Android, Desktop, and Web.";
    pub const BASE_URL: &str = "https://leptos.ui.zweers.ch";
    // Twitter
    pub const TWITTER_IMAGE: &str = "https://leptos.ui.zweers.ch/twitter-card.webp";
}
