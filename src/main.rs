use std::env;
use serde::Deserialize;
use reqwest::Error;
use rand::{seq::SliceRandom};
use colored::Colorize;

use chrono::Local;

const API_KEY: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InN0YWhtYXhmZmNxYW5raWVudWxoIiwicm9sZSI6ImFub24iLCJpYXQiOjE2NTcwNDAzOTUsImV4cCI6MTk3MjYxNjM5NX0.-YZmaNQcoQXbC0_VZYD_jNuOgVbFEu9fbpL_lRDBIH0";

// TODO
#[allow(unused)]
#[derive(Deserialize, Debug)]
struct Event {
    #[serde(alias = "id")]
    id: String,
    #[serde(alias = "title")]
    title: String,
    #[serde(alias = "slugTitle")]
    slug_title: String,
    #[serde(alias = "otd")]
    otd: String,
    #[serde(alias = "description")]
    description: String,
    #[serde(alias = "category")]
    category: String,
    #[serde(alias = "imgAltText")]
    img_alt_text: Option<String>,
    #[serde(alias = "NSFW")]
    nsfw: bool,
    #[serde(alias = "imgSrc")]
    img_src: String,
    #[serde(alias = "date")]
    date: String,
    #[serde(alias = "sources")]
    sources: String,
}

struct MonthDay {
    month: String,
    day: String
}

fn format_short(event: &&Event) -> String {
    let title = event.title.to_string();
    let title = format!("{}", title.to_string()).bold();
    let otd = event.otd.to_string();
    format!("{title}\n{otd}")
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // arguments collection
    // example: cargo run -- 11 28
    let args: Vec<String> = env::args().collect();
    let today: MonthDay = if args.len() == 1 {
        let date = Local::now();
        MonthDay {
            month: date.format("%m").to_string(),
            day: date.format("%d").to_string()
        }
    } else if args.len() != 3 {
        panic!("either no argument or two arguments (month and date) are to be provided");
    } else {
        let month = &args[1];
        let day = &args[2];

        let month_number = month.parse::<i8>().unwrap();
        let day_number = day.parse::<i8>().unwrap();

        match month_number {
            1...12 => print!("{}", 1),
            _ => panic!("month must be in range 1-12")
        }
        match day_number {
            1...31 => print!("{}", 1),
            _ => panic!("month must be in range 1-12")
        }

        if month.len() != 2    {
            panic!("month must contain two digits, e.g. \"03\" or \"12\"")
        }
        // if month.chars().nth(0).unwrap() != "b" {
        //      panic!("noooo");
        // };

        MonthDay {
            month: month.to_string(),
            day: day.to_string()
        }
    };

    // query database
    let request_url = format!("https://stahmaxffcqankienulh.supabase.co/rest/v1/eventLibrary?select=*&date=like.%25{month}%2F{day}%2F%25",
                              month = today.month,
                              day = today.day
                              );
    // println!("{}", request_url);
    let client = reqwest::Client::new();
    let response = client.get(&request_url)
        .header("apikey", API_KEY)
        .send()
        .await?;

    // deserialize
    let events: Vec<Event> = response.json().await?;

    // choose random element
    if !events.is_empty() {
        let random_element: Vec<_> = events.choose_multiple(&mut rand::thread_rng(), 1).collect();
        // unwrap() as it can't fail (see if condition)
        let random_event = random_element.first().unwrap();
        println!("{}", format_short(random_event));
    }
    Ok(())
}
