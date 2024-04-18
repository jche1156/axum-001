use maud::{html, Markup, DOCTYPE};
use postgrest::Postgrest;
use serde::Deserialize;
use serde_json::{Result, Value};

/// A basic header with a dynamic `page_title`.
fn header(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        title { (page_title) }
        link rel="stylesheet" href="assets/app.css";
        script src="https://unpkg.com/htmx.org@1.9.12" integrity="sha384-ujb1lZYygJmzgSwoxRggbCHcjc0rB2XoQrxeTUQyRjrOnlCoYta87iKBWq3EsdM2" crossorigin="anonymous" { }
    }
}

/// A static footer.
fn footer() -> Markup {
    html! {
        footer {
            a href="about" { "About" }
        }
    }
}

/// The final Markup, including `header` and `footer`.
///
/// Additionally takes a `greeting_box` that's `Markup`, not `&str`.
fn page(title: &str, greeting_box: Markup) -> Markup {
    html! {
        // Add the header markup to the page
        (header(title))
        h1 class="text-center" { (title) }
        (greeting_box)
        div hx-post="/rest/hi" hx-trigger="mouseenter" hx-swap="beforebegin" { "[Here Mousey Mousey!]" }
        (footer())
    }
}

#[derive(Debug, Deserialize)]
struct Obj {
    runs: Vec<Run>,
}

#[derive(Debug, Deserialize)]
struct Run {
    id: i8,
    created_at: String,
    date: String,
    distance: String,
    pace: String,
    comments: Option<String>,
    location: String,
}

pub async fn render() -> Markup {
    dotenv::dotenv().ok();
    let db_url = dotenv::var("SUPABASE_URL").unwrap() + "/rest/v1";
    let db_key = dotenv::var("SUPABASE_KEY").unwrap();
    let client = Postgrest::new(db_url).insert_header("apikey", db_key);
    let resp = client.from("run").select("*").execute().await.unwrap();
    let body = resp.text().await.unwrap();
    let runs: Vec<Run> = serde_json::from_str(body.as_str()).unwrap();
    page(
        "Hello!",
        html! {
            div {
                @for item in &runs {
                    div class="bg-red-100 flex justify-center max-w-3xl gap-2 p-4 m-auto" {
                        div class="p-20" {
                            h3 { (item.location) }
                            p { (item.date) }
                        }
                        div {
                            p { (item.distance) }
                            p { (item.pace) }
                        }
                        @match item.comments {
                            None => {
                                p { "No Comment" }
                            },
                            _ => {
                                p { (item.comments.as_ref().unwrap()) }
                            }
                        }
                    }
                }
            }
        },
    )
}