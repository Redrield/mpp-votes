use select::document::Document;
use select::node::Node;
use select::predicate::Class;

mod parser;
mod refine;

pub use self::refine::*;

#[derive(Debug)]
pub struct RawDivision {
    pub date: String,
    pub topic: String,
    pub raw_ayes: String,
    pub raw_nays: String,
}

/// Scrapes the contents of the Votes and Proceedings page for a given day, attempting to determine votes based on certain
/// key words that seem to be consistent in the transcripts.
///
/// Algorithm:
/// 1. Traverse the DOM, keeping track of elements
/// 2. When an element with the text "NAYS" is found, traverse the tree backwards to try to find the subject of the vote
/// The search is a bit messy, but from the pages that I looked at, "moved", "motion", and "reading" seem to be good search terms
/// to find the paragraph where the subject of the vote is declared.
/// 3. Everything between the element containing the vote topic and the NAYS header is the AYE votes, extract those
/// 4. Keep going to get the NAY votes, check to make sure that there isn't a split table (As can occasionally happen on near unanimous motions for AYE)
/// and once there's no sign of continued voting records for the motion in question, parse the data out of the collected DOM nodes.
pub fn extract_votes(date: &str) -> anyhow::Result<Vec<RawDivision>> {
    let page = reqwest::blocking::get(&format!("https://www.ola.org/en/legislative-business/house-documents/parliament-42/session-1/{}/votes-proceedings", date))?.text()?;
    let page = Document::from(page.as_str());

    // Not the best but there isn't a way to go through siblings until the start is found
    let mut seen: Vec<Node> = vec![];
    let mut next_node_has_votes = false;
    let mut check = false;

    let mut division = vec![];

    let mut divisions = vec![];

    for main_table in page.find(Class("drum-table")) {
        for child in main_table.first_child().unwrap().children() {
            // log::info!("DATE {}, TEXT {}", date, child.text());
            if next_node_has_votes {
                // `child` now contains the nays
                division.push(child);
                // corner case: enough nays that the table needs to be split
                next_node_has_votes = false;
                check = true;
                seen.push(child);
                continue;
            }

            // This is the last part of the tally that is consistent.
            if child.text().contains("NAYS") {
                let unanimous = child.text().split_whitespace().any(|s| s == "0");
                // Only add this stuff if its the first occurence of NAY, in the case of a broken table this might be the second time
                if division.is_empty() {
                    // The topic being voted upon
                    let movement = seen
                        .iter()
                        .rfind(|n| {
                            n.text().to_lowercase().contains("motion")
                                || n.text().to_lowercase().contains("moved")
                                || n.text().to_lowercase().contains("reading")
                                || n.text().to_lowercase().contains("bill")
                        })
                        .unwrap_or_else(|| {
                            log::error!("{:#?}", seen);
                            panic!()
                        });
                   division.push(movement.clone());
                    // Everything between now and then (i.e. the AYE votes)
                    division.extend(
                        seen.iter()
                            .rev()
                            .cloned()
                            .take_while(|n| {
                                !(n.text().to_lowercase().contains("motion")
                                    || n.text().to_lowercase().contains("moved")
                                    || n.text().to_lowercase().contains("reading")
                                    || n.text().to_lowercase().contains("bill")
                                )
                            })
                            .collect::<Vec<Node>>()
                            .into_iter()
                            .rev(),
                    );
                    division.push(child);
                }
                if !unanimous {
                    next_node_has_votes = true;
                }
                check = unanimous;
                // This header
                seen.push(child);
                continue;
            }

            // No continuation of NAY votes, send it off
            if check {
                divisions.push(parser::parse_division(&division, date));
                division.clear();
                next_node_has_votes = false;
                check = false;
            }

            seen.push(child)
        }
    }
    Ok(divisions)
}
