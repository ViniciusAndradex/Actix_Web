use std::io;
use actix_web::{web, App, HttpServer, Responder, HttpResponse, get};
use serde::{Deserialize, Serialize};

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(
        || App::new()
            .service(index)
            .service(index_2)
            .service(query_parameter)
    )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[derive(Deserialize, Serialize)]
struct MyInfo {
    id: String,
    username: String
}

#[derive(Deserialize)]
struct User {
    user_id: u8,
    friend: String
}

#[derive(Deserialize, Debug)]
struct QueryParameter {
    #[serde(default)]
    name: Option<String>
}

#[get("/")]
async fn index(path: web::Path<(String, String)>, json: web::Json<MyInfo>) -> impl Responder {
    let path = path.into_inner();
    let ret = format!("{} {} {} {}", path.0, path.1, json.id, json.username);

    HttpResponse::Ok().body(ret)
}

#[get("/user/{user_id}/{friend}")]
async fn index_2(path: web::Path<User>) -> Result<String, io::Error> {
    Ok(format!("Welcome {}, your id is {}", path.friend, path.user_id))
}

#[get("/query")]
async fn query_parameter(query: web::Query<QueryParameter>) -> impl Responder {
    match &query.name {
        Some(x) => HttpResponse::Ok().body(format!("Hello {}", x)),
        None => HttpResponse::Ok().body("You not use query parameter")
    }
}