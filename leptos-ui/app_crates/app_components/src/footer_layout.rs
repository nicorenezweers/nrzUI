use leptos::prelude::*;

#[component]
pub fn FooterLayout() -> impl IntoView {
    view! {
        <footer class="flex z-50 justify-center items-center py-10 px-1 w-full text-sm leading-normal text-center text-pretty text-muted-foreground">
            <p>
                "Built by "
                <a
                    href="https://zweers.ch"
                    target="_blank"
                    rel="noreferrer"
                    class="font-medium underline underline-offset-4"
                >
                    "nrz"
                </a> ". Check our 9-Week Bootcamp "
                <a
                    href="https://zweers.ch"
                    target="_blank"
                    rel="noreferrer"
                    class="font-medium underline underline-offset-4"
                >
                    "here"
                </a> "."
            </p>
        </footer>
    }
}
