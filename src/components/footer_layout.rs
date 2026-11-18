use dioxus::prelude::*;

#[component]
pub fn FooterLayout() -> Element {
    rsx! {
        footer { class: "flex z-50 justify-center items-center py-10 px-1 w-full text-sm leading-normal text-center text-pretty text-muted-foreground",
            p {
                "nrzUI · built by "
                a {
                    href: "https://zweers.ch",
                    target: "_blank",
                    rel: "noreferrer",
                    class: "font-medium underline underline-offset-4",
                    "nrz"
                }
                " · based on "
                a {
                    href: "https://github.com/rust-ui/ui",
                    target: "_blank",
                    rel: "noreferrer",
                    class: "font-medium underline underline-offset-4",
                    "rust-ui"
                }
                " (MIT)."
            }
        }
    }
}
