use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Run {
    pub id: i8,
    created_at: String,
    pub date: String,
    pub distance: String,
    pub pace: String,
    pub comments: Option<String>,
    pub location: String,
}

