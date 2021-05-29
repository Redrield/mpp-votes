use actix_files::NamedFile;
use actix_web::{Result, get};

#[get("/data/members")]
pub async fn members() -> Result<NamedFile> {
    Ok(NamedFile::open("data/members.json")?)
}

#[get("/data/divisions")]
pub async fn divisions() -> Result<NamedFile> {
    Ok(NamedFile::open("data/divisions.json")?)
}