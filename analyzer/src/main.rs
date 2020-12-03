use crate::votes::{refine_division, RawDivision};
use common::Division;
use chrono::{Local, NaiveDate, Datelike, Weekday};
use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;
use std::io;
use rayon::prelude::*;
use itertools::Itertools;

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
    let start_day = std::env::args().nth(1).expect("Start day to analyze required");
    let start = NaiveDate::parse_from_str(&start_day, "%Y-%m-%d").unwrap();
    let today = Local::today().naive_local();

    log::info!("Parsing members page...");
    let members = members::parse_members().await.unwrap();
    log::info!("Members parsed!");

    let divisions = start.iter_days().take_while(|day| *day < today)
        .par_bridge()
        .flat_map(|day| {
            if day.weekday() == Weekday::Fri || day.weekday() == Weekday::Sat || day.weekday() == Weekday::Sun {
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
                        log::error!("Unable to extract the voting record for {}. Error reported: {}", day, e);
                        log::error!("Skipping today.");
                        vec![]
                    }
                }
            }
        }).collect::<Vec<RawDivision>>();

    log::info!("Refining collected divisions...");
    let divisions = divisions.into_iter().map(|div| refine_division(div, &members))
        .filter(|d| !d.topic.is_empty())
        .sorted_by(|div1, div2| Ord::cmp(&div2.date, &div1.date)).collect::<Vec<Division>>();
    log::info!("Refined.");

    log::info!("Updating server..");
    let client = reqwest::Client::default();
    client.post("https://onvotes.ca/api/write/members")
        .body(serde_json::to_string(&members).unwrap())
        .bearer_auth(&token)
        .send()
        .await.unwrap();
    client.post("https://onvotes.ca/api/write/divisions")
        .body(serde_json::to_string(&divisions).unwrap())
        .bearer_auth(&token)
        .send()
        .await.unwrap();
    log::info!("Done.");


//    // Fairly simple stuff here, get the existing records, unpin them to let them slowly be purged from the network, and replace them with the new data
//    let client = IpfsClient::default();
//    log::info!("Resolving old file...");
//    let res = client.name_resolve(Some("/ipns/k2k4r8ka63uxofwvctgqzl9xgz7h8c8sekmhdailvgf1pd9px5bogyxe"), false, false).await.unwrap();
//    log::info!("Resolved path {}", res.path);
//    let redir = String::from_utf8(util::read_complete(client.cat(&res.path)).await).unwrap();
//    let _ = client.pin_rm(&res.path, true).await.unwrap();
//    let mut redir = serde_json::from_str::<Redirects>(&redir).unwrap();
//
//    log::info!("Uploading members...");
//    let members_hash = actions::update_members(members, &client).await;
//
//    let compressed_old = String::from_utf8(util::read_complete(client.cat(&redir.divisions)).await).unwrap();
//    let _ = client.pin_rm(&format!("/ipfs/{}", redir.divisions), true).await.unwrap();
//    // let mut decoder = GzDecoder::new(&compressed_old[..]);
//    // let mut old = String::new();
//    // decoder.read_to_string(&mut old).unwrap();
//    log::info!("Got old divisions. Updating...");
//    let divisions_hash = actions::update_votes(compressed_old, divisions, &client).await;
//
//    log::info!("Updating redirects...");
//    redir.members = members_hash;
//    redir.divisions = divisions_hash;
//    let json = serde_json::to_string(&redir).unwrap();
//    let res = client.add(Cursor::new(json)).await.unwrap();
//    let hash = res.hash;
//
//    // Update the IPNS record of the node hosting this data. The website queries the gateway with IPNS to find the up to date data
//    log::info!("Updating IPNS");
//    let res = client.name_publish(&format!("/ipfs/{}", hash), false, Some("25h"), None, None).await;
//    log::info!("Final result: {}", res.is_ok());
//    log::info!("Done.")
}
