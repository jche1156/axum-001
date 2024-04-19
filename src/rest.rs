use maud::{html, Markup};

pub async fn htmx_msg() -> Markup {
    html! {
        p class="text-2xl text-center" { "Hello! How are you?" }
    }
}
