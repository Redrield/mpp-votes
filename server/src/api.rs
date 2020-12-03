use serde::{Serialize, Deserialize};
use tantivy::schema::Schema;
use tantivy::doc;
use actix_web::{web, get, HttpResponse};
use tantivy::Index;
use tantivy::query::QueryParser;
use tantivy::collector::TopDocs;
use common::{Division, Member};
use actix_web::dev::ServiceRequest;
use actix_web::Result;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Error};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::headers::www_authenticate::bearer::Bearer;
use serde_json::json;
use std::fs::File;

mod riding;
pub use self::riding::lookup_postal_code;

pub async fn validator(req: ServiceRequest, cred: BearerAuth) -> Result<ServiceRequest> {
    let token = std::env::var("API_TOKEN")
        .map_err(|e| HttpResponse::InternalServerError().body(json!({
            "code": 500,
            "response": format!("{}", e),
        })))?;

    if &token == cred.token() {
        Ok(req)
    } else {
        Err(
            AuthenticationError::new(
            Bearer::build()
                .realm("/api/write")
                .error(Error::InvalidToken)
                .finish()
        ).into())
    }
}


#[derive(Serialize)]
pub struct WriteResponse {
    code: u16,
    message: String,
}

pub async fn set_members(data: web::Json<Vec<Member>>) -> HttpResponse {
    println!("Got valid request to set member JSON {:?}", data);
    HttpResponse::Ok().body(json!({
        "code": 200,
        "response": "OK"
    }))
}

pub async fn add_divisions(data: web::Json<Vec<Division>>, schema: web::Data<Schema>, index: web::Data<Index>) -> HttpResponse {

    let mut wr = index.writer(50_000_000).unwrap();
    let date = schema.get_field("date").unwrap();
    let topic = schema.get_field("topic").unwrap();
    let ayes = schema.get_field("ayes").unwrap();
    let nays = schema.get_field("nays").unwrap();

    {
        let mut divs = serde_json::from_reader::<File, Vec<Division>>(File::open("data/divisions.json").unwrap()).unwrap();
        divs.extend_from_slice(&data.0[..]);
        std::fs::write("data/divisions.json", serde_json::to_string(&divs).unwrap()).unwrap();
    }

    for div in data.0 {
        wr.add_document(doc!(
            date => div.date,
            topic => div.topic,
            ayes => serde_json::to_string(&div.ayes).unwrap(),
            nays => serde_json::to_string(&div.nays).unwrap(),
        ));
    }
    wr.commit().unwrap();
    HttpResponse::Ok().finish()
}


#[derive(Deserialize)]
pub struct SearchQuery {
    query: String,
}

#[get("/api/search")]
pub async fn search(query: web::Query<SearchQuery>, schema: web::Data<Schema>, index: web::Data<Index>) -> web::Json<Vec<Division>> {
    let reader = index.reader().unwrap();
    let date = schema.get_field("date").unwrap();
    let topic = schema.get_field("topic").unwrap();
    let ayes = schema.get_field("ayes").unwrap();
    let nays = schema.get_field("nays").unwrap();
    let searcher = reader.searcher();
    let parser = QueryParser::for_index(&index, vec![topic]);
    let query = parser.parse_query(&query.query).unwrap();
    let docs = searcher.search(&query, &TopDocs::with_limit(10)).unwrap();

    web::Json(docs.into_iter().map(|(_, addr)| searcher.doc(addr).unwrap())
        .map(|doc| {
            Division {
                date: doc.get_first(date).unwrap().text().unwrap().to_string(),
                topic: doc.get_first(topic).unwrap().text().unwrap().to_string(),
                ayes: serde_json::from_str(doc.get_first(ayes).unwrap().text().unwrap()).unwrap(),
                nays: serde_json::from_str(doc.get_first(nays).unwrap().text().unwrap()).unwrap()
            }
        })
        .collect())
}