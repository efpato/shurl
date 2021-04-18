use crate::app::db::Pool;
use crate::app::models::{CreateLink, Link};
use crate::app::shortener::Shortener;
use actix_web::{http::header, web, HttpRequest, HttpResponse, Responder};

pub async fn create(
    req: HttpRequest,
    link: web::Json<CreateLink>,
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
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn delete(path: web::Path<(i64,)>, pool: web::Data<Pool>) -> impl Responder {
    let conn = pool.get().expect("Couldn't get db connection!");

    match web::block(move || Link::delete(path.into_inner().0, &conn)).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

pub async fn list(pool: web::Data<Pool>) -> impl Responder {
    let conn = pool.get().expect("Couldn't get db connection!");

    match web::block(move || Link::list(&conn)).await {
        Ok(links) => HttpResponse::Ok().json(links),
        Err(e) => {
            log::error!("{}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn retrieve(path: web::Path<(i64,)>, pool: web::Data<Pool>) -> impl Responder {
    let conn = pool.get().expect("Couldn't get db connection!");

    match web::block(move || Link::find_by_id(path.into_inner().0, &conn)).await {
        Ok(link) => HttpResponse::Ok().json(link),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
