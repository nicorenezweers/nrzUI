use leptos::prelude::*;

use crate::ui::avatar::{Avatar, AvatarFallback, AvatarImage};

#[component]
pub fn DemoAvatar() -> impl IntoView {
    view! {
        <Avatar>
            <AvatarImage attr:src="https://api.dicebear.com/9.x/notionists/svg?seed=nrz" attr:alt="@nrz" />
            <AvatarFallback>"RS"</AvatarFallback>
        </Avatar>
    }
}
