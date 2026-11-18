use leptos::prelude::*;

const LOGO_SRC: &str = "/icons/logo.png";

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="flex flex-col justify-center items-center px-6 h-full text-center">
            <img src=LOGO_SRC alt="nrzUI Logo" class="mb-6 rounded-2xl size-20" />
            <h1 class="text-2xl font-bold tracking-tight">"nrzUI Stack"</h1>
            <p class="mt-2 max-w-xs text-muted-foreground text-pretty">
                "Build cross-platform apps with Rust. By nrz Bootcamp, powered by "
                <a
                    href="https://ui.zweers.ch"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="underline text-primary underline-offset-4 hover:text-primary/80"
                >
                    "nrzUI"
                </a> "."
            </p>
        </div>
    }
}
