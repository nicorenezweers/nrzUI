use dioxus::document::eval;
use dioxus::prelude::*;
use icons::{Moon, Sun};

#[component]
pub fn ThemeToggle() -> Element {
    // nrzUI: dark by default (like all nrzApps); light only when explicitly chosen.
    let mut dark = use_signal(|| true);
    // Don't persist until the stored preference has been read, otherwise the
    // initial default would overwrite it on mount.
    let mut loaded = use_signal(|| false);

    use_effect(move || {
        spawn(async move {
            let mut ev = eval("return localStorage.getItem('darkmode') !== 'false';");
            if let Ok(is_dark) = ev.recv::<bool>().await {
                dark.set(is_dark);
            }
            loaded.set(true);
        });
    });

    // Whenever dark changes (after load): toggle .dark on <html> and persist.
    use_effect(move || {
        let is_dark = dark();
        if !loaded() {
            return;
        }
        eval(&format!(
            "document.documentElement.classList.toggle('dark', {is_dark}); \
             localStorage.setItem('darkmode', '{is_dark}');"
        ));
    });

    let toggle = move |_| {
        dark.set(!dark());
    };

    rsx! {
        button {
            r#type: "button",
            "aria-label": "Toggle theme",
            class: "inline-flex items-center justify-center size-8 rounded-md hover:bg-accent transition-colors cursor-pointer",
            onclick: toggle,
            if dark() {
                Sun { class: "size-4" }
            } else {
                Moon { class: "size-4" }
            }
        }
    }
}
