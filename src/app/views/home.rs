use crate::app::db::Pool;
use crate::app::models::Link;
use crate::app::shortener::Shortener;
use actix_web::{http::header, web, HttpResponse, Responder};

pub async fn follow_link(
    hash: web::Path<String>,
    pool: web::Data<Pool>,
    shortener: web::Data<Shortener>,
) -> impl Responder {
    let conn = pool.get().expect("Couldn't get db connection!");

    match shortener.decode(hash.into_inner()) {
        Ok(id) => match web::block(move || Link::find_by_id(id, &conn)).await {
            Ok(link) => HttpResponse::Found()
                .header(header::LOCATION, link.url)
                .finish(),
            Err(_) => HttpResponse::NotFound().finish(),
        },
        Err(e) => {
            log::error!("{:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
