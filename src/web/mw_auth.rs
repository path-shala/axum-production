use axum::{http::Request, response::Response, middleware::Next};
use tower_cookies::Cookies;

use crate::{web::AUTH_TOKEN, Error, Result};
// use crate::Error::AuthTokenNotFound;


pub async fn mw_require_auth<B>(
    cookies:Cookies,
    req: Request<B>,
    next: Next<B>
    ) -> Result<Response>{

        let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
        auth_token.ok_or(Error::AuthTokenNotFound)?;
    Ok (next.run(req).await)

}
