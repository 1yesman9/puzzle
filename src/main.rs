use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use std::path::PathBuf;

async fn index(_req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "./index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

async fn css(_req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "./index.css".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

async fn audio(_req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "./audio/prelude.wav".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

async fn image(_req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "./images/play_button.png".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| 
            App::new()
                .route("/", web::get().to(index))
                .route("/index.css", web::get().to(css))
                .route("/images/play_button.png", web::get().to(image))
                .route("/audio/prelude.wav", web::get().to(audio))
            )
        .bind("167.172.129.255:443")?
        .run()
        .await
}