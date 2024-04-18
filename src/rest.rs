use maud::{html, Markup};

pub async fn htmx_msg() -> Markup {
    html! {
        p class="text-5xl" { "Hello! How are you?" }
    }
}
