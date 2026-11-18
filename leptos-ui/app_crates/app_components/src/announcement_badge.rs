use icons::ArrowRight;
use leptos::prelude::*;
use registry::ui::button::{Button, ButtonSize, ButtonVariant};

#[component]
pub fn AnnouncementBadge() -> impl IntoView {
    view! {
        <Button size=ButtonSize::Badge variant=ButtonVariant::Secondary attr:href="https://git.zweers.ch/nicorenezweers/nrzUI">
            <span>Now available: nrzUI CLI</span>
            <ArrowRight />
        </Button>
    }
}
