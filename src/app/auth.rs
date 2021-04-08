use super::AppData;
use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::{
    bearer::{BearerAuth, Config},
    AuthenticationError,
};

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);

    match req.app_data::<AppData>() {
        Some(app_data) => {
            if credentials.token() == app_data.token.as_str() {
                Ok(req)
            } else {
                Err(AuthenticationError::from(config).into())
            }
        }
        None => Err(AuthenticationError::from(config).into()),
    }
}
