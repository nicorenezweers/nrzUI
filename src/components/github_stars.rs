use dioxus::prelude::*;
use registry::ui::skeleton::Skeleton;

// Star count of the nrzUI repository on nrzGit (Forgejo-compatible API).
const GITHUB_REPO_URL: &str = "https://git.zweers.ch/nicorenezweers/nrzUI";

/// nrzGit base URL, overridable via `NRZGIT_URL` (same convention as the other nrzApps).
#[cfg(feature = "server")]
fn repo_api_url() -> String {
    let base = std::env::var("NRZGIT_URL")
        .ok()
        .map(|v| v.trim().trim_end_matches('/').to_string())
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| "https://git.zweers.ch".to_string());
    format!("{base}/api/v1/repos/nicorenezweers/nrzUI")
}

#[cfg(feature = "server")]
#[derive(serde::Deserialize)]
struct GithubRepoResponse {
    #[serde(alias = "stars_count", alias = "stars")]
    stargazers_count: u32,
}

#[cfg(feature = "server")]
static STARS_CACHE: std::sync::OnceLock<std::sync::Mutex<Option<(u32, std::time::Instant)>>> =
    std::sync::OnceLock::new();

// `unused_unit` fires inside the `#[server]` macro expansion, not our code.
#[allow(clippy::unused_unit, clippy::missing_errors_doc)] // `#[server]` macro owns adapter/error boundary.
#[server]
async fn fetch_github_stars() -> Result<u32, ServerFnError> {
    const CACHE_TTL_SECS: u64 = 3600;

    let cache = STARS_CACHE.get_or_init(|| std::sync::Mutex::new(None));

    if let Some((count, fetched_at)) = *cache.lock().map_err(|e| ServerFnError::new(e.to_string()))?
        && fetched_at.elapsed().as_secs() < CACHE_TTL_SECS
    {
        return Ok(count);
    }

    let client = reqwest::Client::new();
    let mut request = client.get(repo_api_url()).header("User-Agent", "nrzui");
    if let Ok(token) = std::env::var("NRZGIT_TOKEN") {
        request = request.header("Authorization", format!("token {token}"));
    }

    let response = request.send().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    let repo: GithubRepoResponse = response.json().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    *cache.lock().map_err(|e| ServerFnError::new(e.to_string()))? =
        Some((repo.stargazers_count, std::time::Instant::now()));

    Ok(repo.stargazers_count)
}

#[component]
pub fn GithubStars() -> Element {
    let stars = use_resource(fetch_github_stars);

    rsx! {
        a {
            href: GITHUB_REPO_URL,
            target: "_blank",
            rel: "noopener noreferrer",
            "aria-label": "nrzUI on nrzGit",
            class: "inline-flex items-center gap-1.5 text-sm text-muted-foreground hover:text-foreground transition-colors",
            GithubIcon {}
            match &*stars.read() {
                Some(Ok(count)) => {
                    let formatted = if *count >= 1000 {
                        format!("{}k", count / 1000)
                    } else {
                        count.to_string()
                    };
                    rsx! { span { "{formatted}" } }
                }
                Some(Err(e)) => {
                    eprintln!("fetch_github_stars failed: {e}");
                    rsx! { Skeleton { class: "w-8 h-4" } }
                }
                None => rsx! { Skeleton { class: "w-8 h-4" } },
            }
        }
    }
}

#[component]
fn GithubIcon() -> Element {
    rsx! {
        icons::GitBranch { class: "size-4" }
    }
}
