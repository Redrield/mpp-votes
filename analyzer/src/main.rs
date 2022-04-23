use crate::votes::{refine_division, RawDivision};
use chrono::{Datelike, Local, NaiveDate, Weekday};
use common::Division;
use fern::colors::{Color, ColoredLevelConfig};
use itertools::Itertools;
use log::LevelFilter;
use rayon::prelude::*;
use std::fs::File;
use std::io;

mod members;
mod sites;
mod votes;

fn init_logger() {
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .error(Color::Red)
        .warn(Color::Yellow)
        .debug(Color::White);
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} {} {}  {}",
                Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                colors.color(record.level()),
                record.target(),
                message
            ))
        })
        .level(LevelFilter::Info)
        .chain(io::stdout())
        .apply()
        .unwrap();
}

#[tokio::main]
async fn main() {
    init_logger();
    let token = std::env::var("API_TOKEN").expect("Need token to update server");
    let start_day = std::env::args()
        .nth(1)
        .expect("Start day to analyze required");
    let start = NaiveDate::parse_from_str(&start_day, "%Y-%m-%d").unwrap();
    let today = Local::today().naive_local();

    log::info!("Parsing members page...");
    let members = members::parse_members().await.unwrap();
    log::info!("Members parsed!");

    let divisions = start
        .iter_days()
        .take_while(|day| *day < today)
        .par_bridge()
        .flat_map(|day| {
            if day.weekday() == Weekday::Fri
                || day.weekday() == Weekday::Sat
                || day.weekday() == Weekday::Sun
            {
                log::info!("Skipping {}, house does not sit.", day);
                vec![]
            } else {
                log::info!("Parsing voting record for {}...", day);
                let fmt = day.format("%Y-%m-%d").to_string();
                match votes::extract_votes(&fmt) {
                    Ok(todays_divisions) => {
                        log::info!("Done.");
                        todays_divisions
                    }
                    Err(e) => {
                        log::error!(
                            "Unable to extract the voting record for {}. Error reported: {}",
                            day,
                            e
                        );
                        log::error!("Skipping today.");
                        vec![]
                    }
                }
            }
        })
        .collect::<Vec<RawDivision>>();

    log::info!("Refining collected divisions...");
    let divisions = divisions
        .into_iter()
        .map(|div| refine_division(div, &members))
        .filter(|d| !d.topic.is_empty())
        .sorted_by(|div1, div2| Ord::cmp(&div2.date, &div1.date))
        .collect::<Vec<Division>>();
    log::info!("Refined.");

    // For weekends
    if !members.is_empty() && !divisions.is_empty() {
        log::info!("Updating server...");
        let client = reqwest::Client::default();
        client
            .post("https://onvotes.ca/api/write/members")
            .json(&members)
            .bearer_auth(&token)
            .send()
            .await
            .unwrap();
        client
            .post("https://onvotes.ca/api/write/divisions")
            .json(&divisions)
            .bearer_auth(&token)
            .send()
            .await
            .unwrap();
    }
    log::info!("Done.");
}
