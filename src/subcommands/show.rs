extern crate reqwest;
extern crate serde_json;

use regex::Regex;
use std::process::exit;

pub fn show<S: Into<String>>(owner: S, repo: S, number: i32) {
    let mut response = reqwest::get(&format!("https://api.github.com/repos/{}/{}/issues/{}", owner.into(), repo.into(), number)).unwrap();
    if !response.status().is_success() {
        println!("failed to get issues.");

        if cfg!(test) {
            panic!();
        }

        exit(1);
    }

    let text = response.text().unwrap();
    let json = serde_json::from_str::<serde_json::Value>(&text).unwrap();
    println!("{}\t{} ({})\n\n{}",
             json["number"],
             json["title"].as_str().unwrap(),
             json["html_url"].as_str().unwrap(),
             Regex::new("(?m)^").unwrap().replace_all(json["body"].as_str().unwrap(), "\t"));
}
