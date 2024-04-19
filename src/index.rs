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
        script src="assets/htmx.min.js" { }
        // script src="https://unpkg.com/htmx.org@1.9.12" integrity="sha384-ujb1lZYygJmzgSwoxRggbCHcjc0rB2XoQrxeTUQyRjrOnlCoYta87iKBWq3EsdM2" crossorigin="anonymous" { }
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
        div class="text-center pb-20" hx-post="/rest/hi" hx-trigger="mouseenter" hx-swap="beforebegin" { "[Here Mousey Mousey!]" }
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
    let offline = true;
    let mut body;
    if offline {
        body = String::from(r#"
            [{"id":61,"created_at":"2024-04-16T23:30:21.974691+00:00","date":"2024-04-16","distance":"4","pace":"11:12","comments":"whoooosh","location":"CCSF Track"},
             {"id":2,"created_at":"2024-04-14T19:29:38+00:00","date":"2024-04-14","distance":"1.2","pace":"13:11","comments":null,"location":"City College"},
             {"id":1,"created_at":"2024-04-16T19:29:26.709337+00:00","date":"2024-04-16","distance":"3.2","pace":"12:44","comments":null,"location":"Golden Gate Park"},
             {"id":62,"created_at":"2024-04-16T23:30:23.620244+00:00","date":"2024-04-16","distance":"4","pace":"11:12","comments":"whoooosh","location":"CCSF Track"},
             {"id":63,"created_at":"2024-04-16T23:30:49.220333+00:00","date":"2024-04-16","distance":"4","pace":"11:12","comments":"whoooosh","location":"CCSF Track"},
             {"id":64,"created_at":"2024-04-16T23:30:53.145812+00:00","date":"2024-04-16","distance":"4","pace":"11:12","comments":"whoooosh","location":"CCSF Track"},
             {"id":65,"created_at":"2024-04-16T23:30:54.256464+00:00","date":"2024-04-16","distance":"4","pace":"11:12","comments":"whoooosh","location":"CCSF Track"},
             {"id":66,"created_at":"2024-04-16T23:30:55.287444+00:00","date":"2024-04-16","distance":"4","pace":"11:12","comments":"whoooosh","location":"CCSF Track"},
             {"id":67,"created_at":"2024-04-16T23:30:56.409286+00:00","date":"2024-04-16","distance":"4","pace":"11:12","comments":"whoooosh","location":"CCSF Track"},
             {"id":68,"created_at":"2024-04-16T23:30:57.658397+00:00","date":"2024-04-16","distance":"4","pace":"11:12","comments":"whoooosh","location":"CCSF Track"},
             {"id":69,"created_at":"2024-04-16T23:30:58.771858+00:00","date":"2024-04-16","distance":"4","pace":"11:12","comments":"whoooosh","location":"CCSF Track"},
             {"id":70,"created_at":"2024-04-16T23:36:24.772319+00:00","date":"2024-04-16","distance":"4","pace":"11:12","comments":"whoooosh","location":"CCSF Track"},
             {"id":71,"created_at":"2024-04-16T23:36:25.702372+00:00","date":"2024-04-16","distance":"4","pace":"11:12","comments":"whoooosh","location":"CCSF Track"},
             {"id":72,"created_at":"2024-04-16T23:36:27.012318+00:00","date":"2024-04-16","distance":"4","pace":"11:12","comments":"whoooosh","location":"CCSF Track"}]
             "#);
    } else {
        let db_url = dotenv::var("SUPABASE_URL").unwrap() + "/rest/v1";
        let db_key = dotenv::var("SUPABASE_KEY").unwrap();
        let client = Postgrest::new(db_url).insert_header("apikey", db_key);
        let resp = client.from("run").select("*").execute().await.unwrap();
        body = resp.text().await.unwrap();
    }
    let runs: Vec<Run> = serde_json::from_str(body.as_str()).unwrap();
    page(
        "Hello!",
        html! {
            div class="flex flex-col gap-y-4 items-stretch w-3/5 m-auto max-w-3xl" {
                @for item in &runs {
                    div class="bg-orange-200 text-slate-800 outline-blue-500 h-60 p-8 rounded-lg grid grid-cols-3 grid-rows-3" {
                        div class="col-span-3 flex italic outline outline-2" {
                            h3 class="grow text-3xl" { (item.location) }
                            p class="justify-self-end"{ (item.date) }
                        }
                        div class="row-start-3 outline outline-2" {
                            p { (format!("{} mi", item.distance)) }
                            p { (format!("{} mins / mi", item.pace)) }
                        }
                        p class="row-start-3 col-start-3 outline outline-3" {
                            "Comment: "
                            @match &item.comments {
                                Some(x) => (x),
                                None => ""
                            }
                        }
                    }
                }
            }
        },
    )
}
