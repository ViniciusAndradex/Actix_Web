use actix_web::{body::BoxBody, Either, web, App, HttpServer, Error, Responder, HttpRequest, HttpResponse, get};
use actix_web::http::header::ContentType;
use serde::Serialize;
use futures::{ future::ok, stream::once };

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(web::resource("/")
            .route(web::get().to(index)))
        .service(stream)
        .service(stream2)
    )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

async fn index(_req: HttpRequest) -> String { // Permitido pois String pode ser convertido em Responder
    "Hello world!".to_string()
}

#[derive(Serialize)]
struct SerializeResponse {
    name: String
}

impl Responder for SerializeResponse {
    type Body = BoxBody;
    // Estamos criando uma alias para esse tipo personalizado

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

async fn index2() -> impl Responder {
    SerializeResponse {
        name: "John".to_string()
    }
}

#[get("/stream")]
async fn stream() -> impl Responder {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"hello, john")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

#[get("/stream2")]
async fn stream2() -> impl Responder {
    HttpResponse::Ok().body("hello, john")
}

type RegisterResult = Either<HttpResponse, Result<&'static str, Error>>;

async fn either() -> RegisterResult {
    if is_a_variant() {
        Either::Left(HttpResponse::BadRequest().body("Bad data"))
    } else {
        Either::Right(Ok("Hello!"))
    }
}

fn is_a_variant() -> bool {
    todo!()
}
