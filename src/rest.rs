use maud::{html, Markup};

use crate::schema::Run;

mod components;

pub async fn htmx_msg() -> Markup {
    let item: Run = serde_json::from_str(r#"
        {"id":1,"created_at":"2024-04-16T19:29:26.709337+00:00","date":"2024-04-16","distance":"3.2","pace":"12:44","comments":"This comment should be edited","location":"Golden Gate Park"}             
    "#).unwrap();
    components::run_card(item, true)
}

pub async fn edit() -> Markup {
    html! {
        div class="flex flex-col space-y-2 w-1/4 m-auto"{
            button class="outline outline-3 bg-green-100 hover:bg-green-200 px-2 max-w-24 rounded-sm" hx-post="/rest/confirm" hx-trigger="click" hx-target="closest div" hx-swap="outerHTML" {
                "Confirm"
            }
            button class="outline outline-3 bg-orange-100 hover:bg-orange-200 px-2 max-w-24 rounded-sm" hx-post="/rest/confirm" hx-trigger="click" hx-target="closest div" hx-swap="outerHTML" {
                "Cancel"
            }
        }
    }
}

pub async fn delete() -> Markup {
    html! {
        div class="flex flex-col space-y-2 w-1/4 m-auto"{
            button class="outline outline-3 bg-red-200 hover:bg-red-300 px-2 max-w-24 rounded-sm" hx-post="/rest/confirm" hx-trigger="click" hx-target="closest div" hx-swap="outerHTML" {
                "Confirm"
            }
            button class="outline outline-3 bg-orange-100 hover:bg-orange-200 px-2 max-w-24 rounded-sm" hx-post="/rest/confirm" hx-trigger="click" hx-target="closest div" hx-swap="outerHTML" {
                "Cancel"
            }
        }
    }
}

pub async fn confirm() -> Markup {
    html! {
        div class="flex flex-col space-y-2 w-1/4 m-auto"{
            button class="outline outline-3 hover:bg-orange-200 px-2 max-w-24 rounded-sm" hx-post="/rest/edit" hx-trigger="click" hx-target="closest div" hx-swap="outerHTML" {
                "Edit"
            }
            button class="outline outline-3 hover:bg-red-300 px-2 max-w-24 rounded-sm" hx-post="/rest/delete" hx-trigger="click" hx-target="closest div" hx-swap="outerHTML" {
                "Delete"
            }
        }
    }
}
