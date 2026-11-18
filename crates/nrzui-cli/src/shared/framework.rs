use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};

#[derive(
    Debug, Clone, Copy, Default, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Display, EnumString, AsRefStr,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Framework {
    #[default]
    Leptos,
    Dioxus,
}

impl Framework {
    /// Environment variable that overrides the site URL (e.g. a self-hosted nrzUI instance).
    const fn env_var(self) -> &'static str {
        match self {
            Framework::Leptos => "NRZUI_LEPTOS_URL",
            Framework::Dioxus => "NRZUI_URL",
        }
    }

    const fn default_site_url(self) -> &'static str {
        match self {
            Framework::Leptos => "https://leptos.ui.zweers.ch",
            Framework::Dioxus => "https://ui.zweers.ch",
        }
    }

    /// Site URL, overridable via `NRZUI_URL` (Dioxus) / `NRZUI_LEPTOS_URL` (Leptos).
    pub fn site_url(self) -> String {
        std::env::var(self.env_var())
            .ok()
            .map(|v| v.trim().trim_end_matches('/').to_string())
            .filter(|v| !v.is_empty())
            .unwrap_or_else(|| self.default_site_url().to_string())
    }

    pub fn registry_base_url(self) -> String {
        format!("{}/registry", self.site_url())
    }
}
