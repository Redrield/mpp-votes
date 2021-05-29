use actix_web::{HttpRequest, Result, HttpServer, App, web, get, middleware};
use std::path::{PathBuf, Path};
use actix_files::NamedFile;
use actix_web_httpauth::middleware::HttpAuthentication;

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

    let divisions_path = Path::new("./data/divisions.json");

    HttpServer::new(move || {
        let (schema, division_index) = db::init(divisions_path);
        let auth = HttpAuthentication::bearer(api::validator);

        App::new()
            .wrap(middleware::Logger::default())
            .data(schema)
            .data(division_index)
            .service(index)
            .service(json::members)
            .service(json::divisions)
            .service(api::search)
            .service(api::lookup_postal_code)
            .service(web::scope("/api/write")
                .wrap(auth)
                .route("/members", web::post().to(api::set_members))
                .route("/divisions", web::post().to(api::add_divisions))
            )
            .route("/{filename:.*}", web::get().to(assets))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
