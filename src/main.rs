use std::io;
use std::{
    cell::Cell,
    sync::atomic::{AtomicUsize, Ordering},
    sync::Arc
};
use actix_web::{web, App, HttpServer, Responder, HttpResponse, get, post ,error};
use serde::{Deserialize, Serialize};

#[actix_web::main]
async fn main() -> io::Result<()> {
    let data = AppState { local_count: Cell::new(0),
        global_count: Arc::new(AtomicUsize::new(0))
    };
    HttpServer::new( move || {
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });
        App::new()
            .service(index)
            .service(index_2)
            .service(query_parameter)
            .service(
                web::resource("/post")
                    .app_data(json_config)
                    .route(web::post().to(post_json))
            )
            .service(form_data)
            .app_data(web::Data::new(data.clone()))
            .service(show_count)
            .service(add_one)
    })
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

#[derive(Deserialize)]
struct PostJson {
    id: u8,
    name: String,
    age: u8,
    #[serde(default)]
    address: Option<String>
}

#[derive(Deserialize)]
struct FormData {
    username: String,
    password: String
}

#[derive(Clone)]
struct AppState {
    local_count: Cell<usize>,
    global_count: Arc<AtomicUsize>
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

// #[post("/post")]
async fn post_json(json: web::Json<PostJson>) -> impl Responder {
    match &json.address {
        Some(x) => HttpResponse::Created().body(format!("Hello {}, your id is {} and your age is {}, your address is {}", json.name, json.id, json.age, x)),
        None => HttpResponse::Created().body(format!("Your Address is {:?}", json.address))
    }
}

#[post("/form")]
async fn form_data(form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Created().body(format!("Hello {}, your password is {}", form.username, form.password))
}

#[get("/count")]
async fn show_count(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(format!("Show local count: {}, global_count: {}",
                                    data.local_count.get(),
                                    data.global_count.load(Ordering::Relaxed)))
}

#[post("/count/add_one")]
async fn add_one(data: web::Data<AppState>) -> impl Responder {
    data.global_count.fetch_add(1, Ordering::Relaxed);

    let count = data.local_count.get();
    data.local_count.set(count + 1);
    HttpResponse::Ok()
}