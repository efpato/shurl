use super::views;
use actix_web::{dev::ServiceRequest, web, Error};
use actix_web_httpauth::extractors::{
    bearer::{BearerAuth, Config},
    AuthenticationError,
};
use actix_web_httpauth::middleware::HttpAuthentication;

async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let username = std::env::var("APP_USER").expect("APP_USER not found");
    let password = std::env::var("APP_PASS").expect("APP_PASS not found");
    let token = base64::encode(format!("{}:{}", username, password));

    if credentials.token() == token.as_str() {
        Ok(req)
    } else {
        let config = req
            .app_data::<Config>()
            .map(|data| data.clone())
            .unwrap_or_else(Default::default);

        Err(AuthenticationError::from(config).into())
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(validator);

    cfg.service(web::resource("/").route(web::get().to(views::index)))
        .service(web::resource("/{hash}").route(web::get().to(views::follow_link)))
        .service(
            web::resource("/api/links")
                .wrap(auth)
                .route(web::post().to(views::create_link)),
        );
}
