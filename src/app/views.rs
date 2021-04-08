use super::{
    db::Pool,
    models::{Link, LongLinkDTO},
    shortener::Shortener,
};
use actix_web::{http::header, http::StatusCode, web, HttpRequest, HttpResponse, Responder};

pub async fn create_link(
    req: HttpRequest,
    link: web::Json<LongLinkDTO>,
    pool: web::Data<Pool>,
    shortener: web::Data<Shortener>,
) -> impl Responder {
    let host = req.headers().get(header::HOST).unwrap().to_str().unwrap();
    let conn = pool.get().expect("Couldn't get db connection!");

    match web::block(move || Link::insert(link.into_inner(), &conn)).await {
        Ok(link) => {
            let hash = shortener.encode(link.id);
            HttpResponse::Ok().body(format!("http://{}/{}", host, hash))
        }
        Err(e) => {
            log::error!("{}", e);
            HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).finish()
        }
    }
}

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
