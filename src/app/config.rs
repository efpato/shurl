use super::views;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(views::index))
            .route(web::post().to(views::create_link)),
    )
    .service(web::resource("/{hash}").route(web::get().to(views::follow_link)));
}
