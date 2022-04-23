use crate::sites;
use common::{Member, Party};
use select::document::Document;
use select::predicate::{Class, Name};
use std::iter::Iterator;

/// Members page displays name as <last>, [Hon.] <first>, turn that into <first> <last> for display by the website.
fn reassemble_name(name: &str) -> String {
    let reasssembled = name.rsplit(", ").collect::<Vec<&str>>().join(" ");

    if reasssembled.starts_with("Hon. ") {
        (&reasssembled[5..]).to_string()
    } else {
        reasssembled
    }
}

/// Scrapes the contents of the OLA's webpage of current members, extracting key details and serializing them for use
/// by both the program (in analyzing votes) and the website.
pub async fn parse_members() -> anyhow::Result<Vec<Member>> {
    let body = reqwest::get(sites::MEMBERS).await?.text().await?;
    let body = Document::from(body.as_str());

    let mut members = vec![];

    for node in body.find(Name("tbody")).flat_map(|node| node.children()) {
        if node.html().trim() == "" {
            continue;
        }
        let name = node
            .find(Class("views-field-field-full-name-by-last-name"))
            .next()
            .unwrap();
        let riding = node.find(Class("views-field-name")).next().unwrap();
        let party = node.find(Class("views-field-field-party")).next().unwrap();
        members.push(Member {
            full_name: reassemble_name(name.text().trim()).to_string(),
            // Normalize all dashes for comparsion later on
            riding: riding.text().trim().replace("—", "-"),
            party: Party::from(party.text().trim()),
        });
    }

    Ok(members)
}
