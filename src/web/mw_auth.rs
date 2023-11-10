use axum::{http::Request, response::Response, middleware::Next};
use tower_cookies::Cookies;
use lazy_regex::regex_captures;
use crate::{web::AUTH_TOKEN, Error, Result};
// use crate::Error::AuthTokenNotFound;


pub async fn mw_require_auth<B>(
    cookies:Cookies,
    req: Request<B>,
    next: Next<B>
    ) -> Result<Response>{
        println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");

        let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
        auth_token.ok_or(Error::AuthTokenNotFound)
        .and_then(parse_token)?;
    Ok (next.run(req).await)

}


fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(
		r#"^user-(\d+)\.(.+)\.(.+)"#,
		&token
	)
	.ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id: u64 = user_id.parse().map_err(|_| Error::AuthFailTokenWrongFormat)?;
    Ok((user_id, exp.to_string(), sign.to_string()))

}
