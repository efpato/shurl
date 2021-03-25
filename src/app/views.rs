use super::db::Pool;
use super::models::{Link, LongLinkDTO};
use super::shortener::Shortener;
use actix_web::{http::header, http::StatusCode, web, HttpRequest, HttpResponse, Responder};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("It's a urlshortener service.")
}

pub async fn create_link(
    request: HttpRequest,
    link: web::Json<LongLinkDTO>,
    pool: web::Data<Pool>,
    shortener: web::Data<Shortener>,
) -> impl Responder {
    let host = request
        .headers()
        .get(header::HOST)
        .unwrap()
        .to_str()
        .unwrap();

    match Link::insert(link.into_inner(), &pool.get().unwrap()) {
        Ok(link) => {
            let hash = shortener.encode(link.id as u64);
            HttpResponse::Ok().body(format!("http://{}/{}", host, hash))
        }
        Err(_) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).finish(),
    }
}

pub async fn follow_link(
    hash: web::Path<String>,
    pool: web::Data<Pool>,
    shortener: web::Data<Shortener>,
) -> impl Responder {
    match shortener.decode(hash.into_inner()) {
        Ok(id) => match Link::find_by_id(id as i64, &pool.get().unwrap()) {
            Ok(link) => HttpResponse::Found()
                .header(header::LOCATION, link.url)
                .finish(),
            Err(_) => HttpResponse::NotFound().finish(),
        },
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
