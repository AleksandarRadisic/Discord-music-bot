use std::process::Command;

use regex::Regex;

pub fn parse_query(query: &String) -> Vec<String> {
    if !query.starts_with("http") || !query.contains("&list"){
        return vec![query.to_string()];
    }
    let get_raw_list = Command::new("yt-dlp")
        .args(["-j", "--flat-playlist", &query])
        .output();
    let raw_list = match get_raw_list {
        Ok(list) => String::from_utf8(list.stdout).unwrap(),
        Err(_) => String::from("Error!"),
    };
    let re =
        Regex::new(r#""url":\s*"(https://www\.youtube\.com/watch\?v=[A-Za-z0-9_-]{11})""#).unwrap();
    let urls: Vec<String> = re
        .captures_iter(&raw_list)
        .map(|cap| cap[1].to_string())
        .collect();
    urls
}
