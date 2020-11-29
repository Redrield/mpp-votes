use seed::browser::fetch::Request;
use seed::fetch::Method;
use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};
use serde::Deserialize;

const IPFS_GATEWAY_BASE: &'static str = "https://ipfs.io";
const RIDING_LOOKUP_BASE: &'static str = "https://voterinformationservice.elections.on.ca/api/electoral-district-search/en/postal-code/";
const FRAGMENT: &'static AsciiSet = &CONTROLS.add(b' ');

pub async fn ipfs_get(path: &str) -> Vec<u8> {
    Request::new(format!("{}{}", IPFS_GATEWAY_BASE, path))
        .method(Method::Get)
        .fetch()
        .await.unwrap()
        .bytes().await.unwrap()
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RidingLookupResult {
    electoral_districts: Vec<ElectoralDistrict>,
    poll_division_ids: Vec<u32>,
    postal_code: String,
    stree_name: String,
    street_direction_id: Option<String>,
    street_type_id: String,
    place_name: String,
    street_name_display_text: String,
}

#[derive(Deserialize)]
pub struct ElectoralDistrict {
    election: Option<String>,
    id: u16,
    name: String,
}

pub async fn lookup_postal_code(code: &str) -> String {
    let mut results = Request::new(format!("{}{}", RIDING_LOOKUP_BASE, utf8_percent_encode(code, FRAGMENT)))
        .method(Method::Get)
        .fetch()
        .await.unwrap()
        .json::<Vec<RidingLookupResult>>()
        .await.unwrap();
    let first_result = results.first().unwrap();
    let district = first_result.electoral_districts.first().unwrap();
    district.name.clone()
}