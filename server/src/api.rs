use serde::Deserialize;
use tantivy::schema::Schema;
use actix_web::{web, get};
use tantivy::Index;
use tantivy::query::QueryParser;
use tantivy::collector::TopDocs;
use common::Division;

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