#![cfg_attr(test, allow(unused_imports))]

#[macro_use]
extern crate clap;
extern crate regex;
extern crate reqwest;
extern crate serde_json;

use std::process::Command;
use std::str;
use clap::{ App, ArgMatches };
use regex::Regex;

mod subcommands;
use subcommands::*;

#[cfg(not(test))]
fn main() {
    let yaml = load_yaml!("cli.yml");
    let mut matcher = App::from_yaml(yaml);
    let matches = matcher.clone().get_matches();

    let output = Command::new("git")
        .arg("remote")
        .arg("-v")
        .output()
        .expect("failed to execute process");

    let rg = Regex::new(r"origin\tgit@github.com:(.*)/(.*).git \(fetch\)").unwrap();
    let captured = rg.captures(str::from_utf8(&output.stdout).unwrap()).unwrap();
    let (owner, repo) = (&captured[1], &captured[2]);

    match matches.subcommand() {
        ("list", Some(sub_matches)) => {
            list(owner, repo, &build_query_string(sub_matches));
        },
        ("", None) => {
            matcher.print_help().unwrap();
        }
        _ => unreachable!(),
    }
}

fn build_query_string(matches: &ArgMatches) -> String {
    let query = &matches.args.iter().map(|(name, value)| {
        format!("{}={}", name, value.vals[0].to_str().unwrap())
    }).collect::<Vec<_>>().join("&");
    let mut qs = vec!["", query];
    qs.dedup();
    qs.join("?")
}


#[cfg(test)]
mod test {
    use super::build_query_string;
    use super::App;
    use clap::Arg;

    #[test]
    fn test_build_query_string() {
        let matcher = App::new("test")
            .arg(
                Arg::with_name("opt1")
                    .long("opt1")
                    .takes_value(true)
            )
            .arg(
                Arg::with_name("opt2")
                    .long("opt2")
                    .takes_value(true)
            );

        let no_matches = matcher.clone().get_matches();

        let one_matche = matcher.clone().get_matches_from(vec![
                "test", "--opt1=val1"
            ]);

        let two_matches = matcher.clone().get_matches_from(vec![
                "test", "--opt1=val1", "--opt2=val2"
            ]);

        assert_eq!(build_query_string(&no_matches), "");
        assert_eq!(build_query_string(&one_matche), "?opt1=val1");

        let case_two = build_query_string(&two_matches);
        assert!(case_two == "?opt1=val1&opt2=val2" || case_two == "?opt2=val2&opt1=val1" );
    }
}
