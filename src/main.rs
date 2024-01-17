use std::time::Duration;
use tokio::time::sleep;
use actix_web::{web, HttpResponse, HttpServer, App, Responder, get};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    // para gerar os arquivos de certificado e chave, use o comando abaixo:
    // openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem \
    //   -days 365 -sha256 -subj "/C=CN/ST=Fujian/L=Xiamen/O=TVlinux/OU=Org/CN=muro.lxd"
    // Para remover a senha do arquivo de chave, use o comando abaixo:
    // openssl rsa -in key.pem -out nopass.pem

    HttpServer::new(|| App::new()
        .route("/worker", web::get().to(HttpResponse::Ok))
        .service(index)).workers(4)
        .bind_openssl(("127.0.0.1", 8080), builder)?
        .run()
        .await
}

async fn my_handler() -> impl  Responder {
    sleep(Duration::from_secs(5)).await;

    "Response"
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hy man!")
}