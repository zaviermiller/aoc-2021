extern crate regex;
extern crate reqwest;
extern crate select;

use aoc21::{days, Day};
use regex::Regex;
use scraper::{Html, Selector};
use select::document::Document;
use select::predicate::Name;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Must supply day.");
        return;
    }

    if &args[1] == "run" {
        if args.len() < 4 {
            println!("usage: {} run [day] [part]", &args[0]);
        }
        run_day(
            &args[2].parse::<u8>().unwrap(),
            &args[3].parse::<u8>().unwrap(),
        );
    } else {
        generate_day(&args[1]);
    }

    // run_day(&args[1].parse::<u8>().unwrap());
}

fn run_day(day: &u8, part: &u8) {
    let url: String = format!("https://adventofcode.com/2021/day/{}/input", day);

    let client = reqwest::blocking::Client::new();

    let resp = client
        .get(url)
        .header(
            "cookie",
            format!("session={};", env::var("AOC_COOKIE").unwrap()),
        )
        .send()
        .unwrap();
    assert!(resp.status().is_success());

    let input = resp.text().unwrap();

    let s_day = &days()[(day - 1) as usize];
    match part {
        1 => println!("{}", s_day.part1(&input)),
        2 => println!("{}", s_day.part2(&input)),
        _ => println!("Invalid part"),
    }
}

fn generate_day(day: &String) {
    let url: String = format!("https://adventofcode.com/2021/day/{}", day);

    let mut resp = reqwest::blocking::get(url).unwrap();
    assert!(resp.status().is_success());

    let body = resp.text().unwrap();

    let fragment = Html::parse_document(&body);

    let article = Selector::parse("article.day-desc > h2").unwrap();

    let mut day_text: String = "".to_string();

    for line in fragment.select(&article) {
        day_text = line.text().collect::<String>();
        println!("{}", day_text);
    }

    gen_template(day, &day_text)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn gen_template(day: &String, day_desc: &String) {
    let path_str = format!("./src/day{}.rs", day);
    let path = Path::new(&path_str);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    if let Ok(lines) = read_lines("./day.rs.tmpl") {
        for line in lines {
            if let Ok(text) = line {
                match file.write_all(
                    format!(
                        "{}\n",
                        text.replace("%day%", day).replace("%day_desc%", day_desc)
                    )
                    .as_bytes(),
                ) {
                    Err(why) => panic!("couldn't write to {}: {}", display, why),
                    Ok(_) => println!("successfully wrote to {}", display),
                }
            }
        }
    }
}
