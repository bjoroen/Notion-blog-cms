use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::notion::notion_db::get_notion_db;

pub async fn has_blog_post_changed() -> bool {
    let timestamp_file = File::open("./static/timestamp.json").expect("No such file");
    let buf = BufReader::new(timestamp_file);
    let vec_of_timestamps: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Could not parse this line"))
        .collect();
    let db_data = get_notion_db().await.unwrap();

    if vec_of_timestamps.len() != db_data.len() {
        return true;
    }

    let mut x: Vec<String> = Vec::new();
    for page in &db_data {
        x.push(page.last_edited_time.clone())
    }

    if vec_of_timestamps != x {
        return true;
    }

    false
}
