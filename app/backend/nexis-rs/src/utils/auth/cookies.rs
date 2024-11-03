use crate::prelude::*;
use types::SSS_COOKIE_NAME;
use anyhow::Result;

pub fn get_sss_pub_token(req: HttpRequest) -> Result<String> {
    let sss_pub_token =
        if let Some(sss_pub_cookie) = req.cookie(SSS_COOKIE_NAME) {
            sss_pub_cookie.value().to_string()
        } else {
            bail!("Session cookie is missing.")
        };

    Ok(sss_pub_token)
}