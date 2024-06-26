use maud::{html, Markup, DOCTYPE};
use postgrest::Postgrest;

use crate::schema::Run;

/// A basic header with a dynamic `page_title`.
fn header(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        title { (page_title) }
        link rel="stylesheet" href="assets/app.css";
        script src="assets/htmx.min.js" { }
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
        body class="bg-slate-800"
        {
            (header(title))
            h1 class="my-12 mx-auto w-3/5 text-3xl text-center text-white" { (title) }
            (greeting_box)
            (footer())
        }
    }
}

pub async fn render() -> Markup {
    dotenv::dotenv().ok();
    let offline = true;
    let body;
    if offline {
        body = String::from(
            r#"
            [{"id":61,"created_at":"2024-04-16T23:30:21.974691+00:00","date":"2024-04-13","distance":"2.0","pace":"13:32","comments":"whoooosh","location":"CCSF Track"},
             {"id":2,"created_at":"2024-04-14T19:29:38+00:00","date":"2024-04-14","distance":"1.2","pace":"13:11","comments":null,"location":"City College"},
             {"id":1,"created_at":"2024-04-16T19:29:26.709337+00:00","date":"2024-04-16","distance":"3.2","pace":"12:44","comments":null,"location":"Golden Gate Park"},
             {"id":62,"created_at":"2024-04-16T23:30:23.620244+00:00","date":"2024-04-16","distance":"4.2","pace":"14:32","comments":"I ran a whole lot and got faster today!","location":"CCSF Track"},
             {"id":63,"created_at":"2024-04-16T23:30:49.220333+00:00","date":"2024-04-17","distance":"1.4","pace":"11:12","comments":"Today I saw a rare bird while running!","location":"CCSF Track"},
             {"id":64,"created_at":"2024-04-16T23:30:53.145812+00:00","date":"2024-04-18","distance":"1.0","pace":"11:02","comments":"New personal best - 1 mi","location":"CCSF Track"},
             {"id":65,"created_at":"2024-04-16T23:30:54.256464+00:00","date":"2024-04-19","distance":"0.7","pace":"11:21","comments":"Short run due to weather","location":"CCSF Track"},
             {"id":66,"created_at":"2024-04-16T23:30:55.287444+00:00","date":"2024-04-19","distance":"1.4","pace":"10:05","comments":"whoooosh","location":"CCSF Track"},
             {"id":67,"created_at":"2024-04-16T23:30:56.409286+00:00","date":"2024-04-20","distance":"1.5","pace":"15:22","comments":"Walking around the park","location":"CCSF Track"},
             {"id":68,"created_at":"2024-04-16T23:30:57.658397+00:00","date":"2024-04-20","distance":"1.2","pace":"10:32","comments":"Second run for today","location":"CCSF Track"},
             {"id":69,"created_at":"2024-04-16T23:30:58.771858+00:00","date":"2024-04-21","distance":"4","pace":"11:12","comments":"race day practice","location":"CCSF Track"},
             {"id":70,"created_at":"2024-04-16T23:36:24.772319+00:00","date":"2024-04-21","distance":"4","pace":"11:12","comments":"5km first time","location":"CCSF Track"},
             {"id":71,"created_at":"2024-04-16T23:36:25.702372+00:00","date":"2024-04-22","distance":"4","pace":"11:12","comments":"warm up","location":"CCSF Track"},
             {"id":72,"created_at":"2024-04-16T23:36:27.012318+00:00","date":"2024-04-22","distance":"4","pace":"11:12","comments":"5km second time","location":"CCSF Track"}]
             "#,
        );
    } else {
        let db_url = dotenv::var("SUPABASE_URL").unwrap() + "/rest/v1";
        let db_key = dotenv::var("SUPABASE_KEY").unwrap();
        let client = Postgrest::new(db_url).insert_header("apikey", db_key);
        let resp = client.from("run").select("*").execute().await.unwrap();
        body = resp.text().await.unwrap();
    }
    let runs: Vec<Run> = serde_json::from_str(body.as_str()).unwrap();
    page(
        "Welcome to John's Run Tracker",
        html! {
            div class="flex flex-col gap-y-4 items-stretch m-auto w-3/5 max-w-3xl min-w-80" {
                @for item in &runs {
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
                div class="pb-20 text-3xl text-center text-white" hx-post="/rest/hi" hx-trigger="mouseenter" hx-swap="beforebegin" { "[New Run!]" }
            }
        },
    )
}
