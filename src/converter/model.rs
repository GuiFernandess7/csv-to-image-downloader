use csv::{Reader, ReaderBuilder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    ts: i64,
    image_url: String,
    created_at: String,
    sex: String,
    subreddit: String
}
