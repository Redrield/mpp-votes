use tantivy::{Index, doc};
use std::path::Path;
use std::fs;
use tantivy::schema::*;
use common::Division;

pub fn init(json_path: &Path) -> (Schema, Index) {
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("topic", TEXT | STORED);
    schema_builder.add_text_field("date", TEXT | STORED);
    schema_builder.add_text_field("ayes", STORED);
    schema_builder.add_text_field("nays", STORED);
    let schema = schema_builder.build();

    let index = Index::create_from_tempdir(schema.clone()).unwrap();

    let topic = schema.get_field("topic").unwrap();
    let date = schema.get_field("date").unwrap();
    let ayes = schema.get_field("ayes").unwrap();
    let nays = schema.get_field("nays").unwrap();
    let mut wr = index.writer(50_000_000).unwrap();
    let divs = serde_json::from_str::<Vec<Division>>(&fs::read_to_string(json_path).unwrap()).unwrap();
    for d in divs {
        wr.add_document(doc!(
            topic => d.topic,
            date => d.date,
            ayes => serde_json::to_string(&d.ayes).unwrap(),
            nays => serde_json::to_string(&d.nays).unwrap(),
        ));
    }
    wr.commit().unwrap();

    (schema, index)
}