extern crate reqwest;
extern crate serde_json;

use std::process::exit;

pub fn list<S: Into<String>>(owner: S, repo: S, query_string: S) {
    let mut response = reqwest::get(&format!("https://api.github.com/repos/{}/{}/issues{}", owner.into(), repo.into(), query_string.into())).unwrap();
    if !response.status().is_success() {
        println!("failed to get issues.");

        if cfg!(test) {
            panic!();
        }

        exit(1);
    }

    let text = response.text().unwrap();
    let json = serde_json::from_str::<serde_json::Value>(&text).unwrap();
    for array in json.as_array().unwrap() {
        println!("{}\t{} ({})", array["number"], array["title"].as_str().unwrap(), array["html_url"].as_str().unwrap());
    }
}


#[cfg(test)]
mod tests {
    use super::list;

    #[test]
    fn test_list_success() {
        list("hexium310", "git-issue", "");
    }

    #[test]
    #[should_panic]
    fn test_list_failure() {
        list("hexium310", "none-of-git-issue", "");
    }
}
