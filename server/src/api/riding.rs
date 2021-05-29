use actix_web::{web, HttpResponse, get};
use crate::api::SearchQuery;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use reqwest::Client;

const LOOKUP_BASE: &'static str = "https://voterinformationservice.elections.on.ca/api/electoral-district-search/en/postal-code/";
const FRAGMENT: &'static AsciiSet = &CONTROLS.add(b' ');

#[get("/api/postal_code")]
pub async fn lookup_postal_code(query: web::Query<SearchQuery>) -> HttpResponse {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build().unwrap();
    let res = client.get(&format!("{}{}", LOOKUP_BASE, utf8_percent_encode(&query.query, FRAGMENT)))
        .send()
        .await.unwrap()
        .text()
        .await.unwrap();
    // let client = Client::new();
    // let res= client.get(&format!("{}{}", LOOKUP_BASE, utf8_percent_encode(&query.query, FRAGMENT)))
    //     .send()
    //     .await.unwrap()
    //     .body()
    //     .await.unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(res)
}
