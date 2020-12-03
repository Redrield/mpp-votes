use actix_web::{web, HttpResponse, get};
use crate::api::SearchQuery;
use actix_web::client::Client;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const LOOKUP_BASE: &'static str = "https://voterinformationservice.elections.on.ca/api/electoral-district-search/en/postal-code/";
const FRAGMENT: &'static AsciiSet = &CONTROLS.add(b' ');

#[get("/api/postal_code")]
pub async fn lookup_postal_code(query: web::Query<SearchQuery>) -> HttpResponse {
    let client = Client::new();
    let res= client.get(&format!("{}{}", LOOKUP_BASE, utf8_percent_encode(&query.query, FRAGMENT)))
        .send()
        .await.unwrap()
        .body()
        .await.unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(res)
}
