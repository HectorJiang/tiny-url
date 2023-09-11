use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod api;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(api::links::create_link)
            .service(api::links::get_from_link)
            .service(api::links::get_all_links)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}