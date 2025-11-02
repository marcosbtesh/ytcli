use serde::Deserialize;
use std::process::{Command, Stdio};

#[derive(Debug, Deserialize)]
struct YtDlpOutput {
    title: Option<String>,
    webpage_url: Option<String>,
}

pub fn search(query: &str, page: i16) -> Vec<(String, String)> {
    let per_page = 5;
    let offset = page * per_page;

    let search_arg = format!("ytsearch{}offset{}:{}", per_page, offset, query);

    let output = Command::new("yt-dlp")
        .arg(search_arg)
        .arg("--print-json")
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to run yt-dlp. Is it installed?");

    // if !output.status.success() {
    //     return vec![];
    // }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut results = Vec::new();

    for line in stdout.lines() {
        if let Ok(entry) = serde_json::from_str::<YtDlpOutput>(line) {
            if let (Some(title), Some(url)) = (entry.title, entry.webpage_url) {
                results.push((title, url));
            }
        }
    }

    results
}
