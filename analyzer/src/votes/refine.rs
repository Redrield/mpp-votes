use crate::votes::RawDivision;
use regex::Regex;
use serde::{Serialize, Deserialize};
use itertools::Itertools;
use common::{Member, Division, Party};

/// A function to turn the extracted data around a vote into a more computer workable format
/// This function will iterate over the ayes and nays extracted in the raw division, and match them with the full details for the MPP
/// The resultant Division will contain an identical topic, with aye and nay fields being Vec<Member> instead of raw Strings.
pub fn refine_division(division: RawDivision, all_members: &Vec<Member>) -> Division {
    let pat = Regex::new(r#"\r\n(\r\n)?"#).unwrap();

    let mut ayes = vec![];
    for aye in pat.split(&division.raw_ayes) {
        if let Some(member) = identify_member(aye, &all_members) {
            ayes.push(member);
        }
    }

    let mut nays = vec![];
    for nay in pat.split(&division.raw_nays) {
        if let Some(member) = identify_member(nay, &all_members) {
            nays.push(member);
        }
    }

    Division {
        date: division.date,
        topic: division.topic.replace(r#"\r\n"#, " "),
        ayes,
        nays,
    }
}

/// A function to identify a voting member based on the name provided in the transcript, and the information of sitting members
/// Member names are deduped in the transcript by putting their riding name (or part of it) in parenthesis after the name,
/// if there are parenthesis we assume that there is some disambiguation required, and so search for members based on last name and that their riding contains the text in parens
/// If none then just look by last name.
/// If no members are found, the individual in question isn't sitting anymore, and their vote is discarded from the records.
fn identify_member(name: &str, members: &Vec<Member>) -> Option<Member> {
    if name.contains("(") {
        let mut it = name.split_whitespace();
        let name = it.next().unwrap().trim();
        let riding = it
            .collect::<Vec<&str>>()
            .join(" ")
            .replace("(", "")
            .replace(")", "")
            .replace("–", "-");
        let riding = riding.trim();
        members
            .iter()
            .find(|m| m.last_name() == name.to_string() && m.riding.contains(riding))
            .cloned()
    } else {
        members
            .iter()
            .find(|m| m.last_name() == name.to_string())
            .cloned()
    }
}
