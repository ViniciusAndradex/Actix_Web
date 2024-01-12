use actix_web::{web, get, App, HttpServer, Responder, HttpResponse, guard};
use std::sync::Mutex;

struct AppState {
    app_name: String,
}

// Shared State
struct AppStateWithCounter {
    counter: Mutex<i32> // Garante segurança para compartilhamento entre threads, faz com que seja criado somente uma referencia por vez.
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });
    // let scope = web::scope("/compose/app").service(test_compose_scope_app); Não consegui executar desta forma

    HttpServer::new( move || {
        App::new()
            .configure(config)
            .service(
                web::scope("/app")
                    .route("/index.html", web::get().to(index))
            )
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .service(index_state)
            .app_data(counter.clone())
            .route("/shared_state", web::get().to(index_shared_state))
            .service(
                web::scope("/compose/app").configure(scoped_config) // o configure adiciona um novo endpoint de test para esta rota, o que torna um polimorfismo interessante.
                    .service(test_compose_scope_app))
            .service(
                web::scope("/") // Outros guards podem ser adicionados, por exemplo podemos criar vários else if
                    .guard(guard::Host("127.0.0.1")) // Só será permitdo acessar a requisição se o host for o valor escrito nesta funçãp, se não é bloqueado.
                    .route("", web::to(|| async { HttpResponse::Ok().body("www")}))
            )
            .route("/", web::to(|| async { HttpResponse::Ok().body("Não estou sendo acessado pelo google.")})) // else em relação ao guard
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// Writing An Application
async fn index() -> impl Responder {
    "Hello, world!"
}

// State
#[get("/index_state")]
async fn index_state(data: web::Data<AppState>) -> String {
    let app_state = &data.app_name;
    format!("Hello, {app_state}!")
}

//Shared State
async fn index_shared_state(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Request number: {}", counter)
}

#[get("/test_scope")]
async fn test_compose_scope_app() -> impl Responder {
    HttpResponse::Ok().body(format!("Number: {}, i test scope for compose app and scope", 1))
}

fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| async { HttpResponse::Ok().body("Test") }))
            .route(web::head().to(|| async {  HttpResponse::MethodNotAllowed() }))
    );
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
            .route(web::head().to(|| async { HttpResponse::MethodNotAllowed() }))
    );
}