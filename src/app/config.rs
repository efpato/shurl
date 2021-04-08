use super::{auth, views};
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(auth::validator);

    cfg.service(
        web::scope("/api")
            .wrap(auth)
            .service(web::resource("/links").route(web::post().to(views::create_link))),
    )
    .service(web::resource("/{hash}").route(web::get().to(views::follow_link)));
}
