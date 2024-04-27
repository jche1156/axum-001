use maud::{html, Markup};

use crate::schema::Run;

fn editable_card(item: Run) -> Markup {
    html! {
        div class="grid grid-cols-3 grid-rows-3 p-4 h-64 rounded-lg bg-slate-500 outline-blue-500 run-card" {
            div class="flex col-span-3 italic text-slate-300" {
                input type="text" value=(item.id) disabled class="mt-2 mr-2 ml-1 w-5 h-5 text-xs text-center rounded-xl opacity-55 bg-slate-300 text-slate-500 outline outline-3";
                h3 class="text-3xl grow" { (item.location) }
                p class="justify-self-end"{ (item.date) }
            }
            div class="flex col-span-3 row-start-3 justify-between p-4 my-auto space-x-2 h-20 text-sm italic rounded-lg text-slate-600 bg-slate-300" {
                div class="w-1/3 min-w-20" {
                    p { (format!("{} mi", item.distance)) }
                    p { (format!("{} / mi", item.pace)) }
                }
                p class="inline-block w-full align-middle opacity-70" {
                    "Comment: "
                    @match &item.comments {
                        Some(x) => (x),
                        None => ""
                    }
                }
                div class="flex flex-col m-auto space-y-2 w-1/4"{
                    button class="px-2 bg-green-100 rounded-sm hover:bg-green-200 outline outline-3 max-w-24" hx-post="/rest/confirm" hx-trigger="click" hx-target="closest div" hx-swap="outerHTML" {
                        "Confirm"
                    }
                    button class="px-2 bg-orange-100 rounded-sm hover:bg-orange-200 outline outline-3 max-w-24" hx-post="/rest/confirm" hx-trigger="click" hx-target="closest div" hx-swap="outerHTML" {
                        "Cancel"
                    }
                }
            }
        }
    }
}

fn uneditable_card(item: Run) -> Markup {
    html! {
        div class="grid grid-cols-3 grid-rows-3 p-4 h-64 rounded-lg bg-slate-500 outline-blue-500 run-card" {
            div class="flex col-span-3 italic text-slate-300" {
                input type="text" value=(item.id) disabled class="mt-2 mr-2 ml-1 w-5 h-5 text-xs text-center rounded-xl opacity-55 bg-slate-300 text-slate-500 outline outline-3";
                h3 class="text-3xl grow" { (item.location) }
                p class="justify-self-end"{ (item.date) }
            }
            div class="flex col-span-3 row-start-3 justify-between p-4 my-auto space-x-2 h-20 text-sm italic rounded-lg text-slate-600 bg-slate-300" {
                div class="w-1/3 min-w-20" {
                    p { (format!("{} mi", item.distance)) }
                    p { (format!("{} / mi", item.pace)) }
                }
                p class="inline-block w-full align-middle opacity-70" {
                    "Comment: "
                    @match &item.comments {
                        Some(x) => (x),
                        None => ""
                    }
                }
                div class="flex flex-col m-auto space-y-2 w-1/4"{
                    button class="px-2 rounded-sm hover:bg-orange-200 outline outline-3 max-w-24" hx-post="/rest/edit" hx-trigger="click" hx-target="closest div" hx-swap="outerHTML" {
                        "Edit"
                    }
                    button class="px-2 rounded-sm hover:bg-red-300 outline outline-3 max-w-24" hx-post="/rest/delete" hx-trigger="click" hx-target="closest div" hx-swap="outerHTML" {
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
        false => uneditable_card(item),
    }
}
