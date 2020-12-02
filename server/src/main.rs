use actix_files as fs;
use actix_web::{HttpRequest, HttpResponse, Result, HttpServer, App, web, get, middleware};
use actix_web::http::StatusCode;
use std::path::{PathBuf, Path};
use actix_files::NamedFile;
use common::Member;

mod db;
mod json;
mod api;

async fn assets(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = format!("dist/{}", req.match_info().query("filename")).parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[get("/")]
async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("dist/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    let port = std::env::var("PORT").unwrap_or("8000".to_string());
    let port_ssl = std::env::var("PORT_SSL").unwrap_or("8443".to_string());

    let divisions_path = Path::new("./data/divisions.json");


    let res = HttpServer::new(move || {
        let (schema, division_index) = db::init(divisions_path);

        App::new()
            .wrap(middleware::Logger::default())
            .data(schema)
            .data(division_index)
            .service(index)
            .service(json::members)
            .service(json::divisions)
            .service(api::search)
            .route("/{filename:.*}", web::get().to(assets))
    })
        .bind(&format!("0.0.0.0:{}", port))?
        .run()
        .await;

    log::info!("I can deinit here");
    res
}
