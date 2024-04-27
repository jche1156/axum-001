use maud::{html, Markup};

use crate::schema::Run;

fn editable_card(item: Run) -> Markup {
    html! {
        div class="bg-slate-500 outline-blue-500 h-64 p-4 rounded-lg grid grid-cols-3 grid-rows-3 run-card" {
            div class="col-span-3 text-slate-300 flex italic" {
                input type="text" value=(item.id) disabled class="text-xs rounded-xl opacity-55 bg-slate-300 text-slate-500 outline outline-3 w-5 h-5 ml-1 mt-2 text-center mr-2";
                h3 class="grow text-3xl" { (item.location) }
                p class="justify-self-end"{ (item.date) }
            }
            div class="h-20 p-4 text-sm italic row-start-3 col-span-3 rounded-lg flex my-auto justify-between space-x-2 text-slate-600 bg-slate-300" {
                div class="w-1/3 min-w-20" {
                    p { (format!("{} mi", item.distance)) }
                    p { (format!("{} / mi", item.pace)) }
                }
                p class="w-full inline-block align-middle opacity-70" {
                    "Comment: "
                    @match &item.comments {
                        Some(x) => (x),
                        None => ""
                    }
                }
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
    }
}

fn uneditable_card(item: Run) -> Markup {
    html! {
        div class="bg-slate-500 outline-blue-500 h-64 p-4 rounded-lg grid grid-cols-3 grid-rows-3 run-card" {
            div class="col-span-3 text-slate-300 flex italic" {
                input type="text" value=(item.id) disabled class="text-xs rounded-xl opacity-55 bg-slate-300 text-slate-500 outline outline-3 w-5 h-5 ml-1 mt-2 text-center mr-2";
                h3 class="grow text-3xl" { (item.location) }
                p class="justify-self-end"{ (item.date) }
            }
            div class="h-20 p-4 text-sm italic row-start-3 col-span-3 rounded-lg flex my-auto justify-between space-x-2 text-slate-600 bg-slate-300" {
                div class="w-1/3 min-w-20" {
                    p { (format!("{} mi", item.distance)) }
                    p { (format!("{} / mi", item.pace)) }
                }
                p class="w-full inline-block align-middle opacity-70" {
                    "Comment: "
                    @match &item.comments {
                        Some(x) => (x),
                        None => ""
                    }
                }
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
    }
}

pub fn run_card(item: Run, editable: bool) -> Markup {     
    match editable {
        true => editable_card(item),
        false => uneditable_card(item)
    }
}
