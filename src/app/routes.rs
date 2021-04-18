use super::{auth, views};
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(auth::validator);

    cfg.service(
        web::scope("/api")
            .wrap(auth)
            .service(
                web::resource("/links{_:/?}")
                    .route(web::post().to(views::links::create))
                    .route(web::get().to(views::links::list)),
            )
            .service(
                web::resource("/links/{link_id}{_:/?}")
                    .route(web::get().to(views::links::retrieve))
                    .route(web::delete().to(views::links::delete)),
            ),
    )
    .service(web::resource("/{hash}").route(web::get().to(views::home::follow_link)));
}
