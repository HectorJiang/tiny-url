use actix_web::{App, HttpServer, Responder};

mod api;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .service(api::links::create_link)
            // .service(api::links::get_from_link)
            .service(api::links::get_all_links)
            .service(api::links::test)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}