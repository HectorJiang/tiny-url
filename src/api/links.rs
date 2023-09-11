use actix_web::{get, HttpResponse, post, Responder};
use actix_web::http::header;
use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use nanoid::nanoid;
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Link {
    tiny_code:String,
    origin_url:String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ApiAddLink{
    origin_url:String,
}

impl ApiAddLink{
    fn to_new_link(self)->Link{
        Link{
            tiny_code:nanoid!(5),
            origin_url:self.origin_url,
        }
    }
}

#[post("/create")]
async fn create_link(link: Json<ApiAddLink>) -> impl Responder{
    let new_link = link.0.to_new_link();
    let new_code = new_link.tiny_code.clone();
    Json(new_code)
}

#[get("/{code}")]
async fn get_from_link() -> impl Responder{
    let url = "https://baidu.com";
    HttpResponse::Found().append_header((header::LOCATION, url)).finish()
}

#[get("/links")]
async fn get_all_links() -> impl Responder{
    let mut links = Vec::new();
    links.push(Link{tiny_code:String::from("111"), origin_url:String::from("https://baidu.com")})
    links.push(Link{tiny_code:String::from("222"), origin_url:String::from("https://google.com")})
    Json(links)
}