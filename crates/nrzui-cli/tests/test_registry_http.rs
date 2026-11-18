//! Live tests against the deployed registry (NRZUI_URL, default https://ui.zweers.ch).
//! Ignored by default: run with `cargo test -- --ignored` once the site is online.

use nrzui_cli::shared::framework::Framework;
use nrzui_cli::shared::nrz_ui_client::NrzUiClient;

#[tokio::test]
#[ignore = "needs the deployed registry (network)"]
async fn test_fetch_tree_md() {
    let result = NrzUiClient::fetch_tree_md(Framework::Dioxus).await;

    assert!(result.is_ok(), "Failed to fetch tree.md: {:?}", result.err());

    let content = result.unwrap();
    assert!(!content.is_empty(), "tree.md content should not be empty");
}

#[tokio::test]
#[ignore = "needs the deployed registry (network)"]
async fn test_fetch_styles_default_alert() {
    let result = NrzUiClient::fetch_styles_default("alert", Framework::Dioxus).await;

    assert!(result.is_ok(), "Failed to fetch alert.md: {:?}", result.err());

    let rust_code = result.unwrap();
    assert!(!rust_code.is_empty(), "Extracted Rust code from alert.md should not be empty");
    // Basic sanity check that it contains Rust code
    assert!(
        rust_code.contains("fn") || rust_code.contains("use") || rust_code.contains("pub"),
        "Content should contain Rust code"
    );
}

#[tokio::test]
#[ignore = "needs the deployed registry (network)"]
async fn test_fetch_styles_default_button() {
    let result = NrzUiClient::fetch_styles_default("button", Framework::Dioxus).await;

    assert!(result.is_ok(), "Failed to fetch button.md: {:?}", result.err());

    let rust_code = result.unwrap();
    assert!(!rust_code.is_empty(), "Extracted Rust code from button.md should not be empty");
}

#[tokio::test]
#[ignore = "needs the deployed registry (network)"]
async fn test_fetch_nonexistent_component() {
    let result = NrzUiClient::fetch_styles_default("nonexistent_component_xyz", Framework::Dioxus).await;

    // Should fail for nonexistent components
    assert!(result.is_err(), "Should fail when fetching nonexistent component");
}
