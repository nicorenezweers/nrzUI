use app_config::{BreadcrumbItem, JsonLdBreadcrumb, SeoMeta, SiteConfig};
use dioxus::prelude::*;

use crate::__registry__::all_workflows::ALL_WORKFLOWS;
use crate::domain::workflows::components::workflow_viewer::WorkflowViewer;

#[component]
pub fn WorkflowsPage() -> Element {
    rsx! {
        SeoMeta {
            title: "Workflows · nrzUI Component Library | nrzUI".to_string(),
            description: "Ready-made workflow blocks built with nrzUI components for Dioxus applications."
                .to_string(),
            canonical_url: format!("{}/workflows", SiteConfig::BASE_URL),
        }
        JsonLdBreadcrumb {
            breadcrumbs: vec![
                BreadcrumbItem { name: "Home".to_string(), url: Some(SiteConfig::BASE_URL.to_string()) },
                BreadcrumbItem { name: "Workflows".to_string(), url: None },
            ],
        }
        for workflow in ALL_WORKFLOWS {
            WorkflowViewer { workflow_entry: *workflow }
        }
    }
}
