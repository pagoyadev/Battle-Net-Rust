#![forbid(unsafe_code)]

extern crate thiserror;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Transport: {0}")]
    TransportError(reqwest::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Credentials {
    access_token: String,
    token_type: String,
    expires_in: u32,
    scope: String,
}

const COMMON_AUTH_URI: &str = "https://oauth.battle.net/authorize";
const CHINA_AUTH_URI: &str = "https://oauth.battlenet.com.cn/authorize";

pub enum Region {
    UnitedStates,
    Europe,
    Korea,
    Taiwan,
    China,
}

impl Region {
    pub fn get_auth_uri(&self) -> &'static str {
        match &self {
            Region::China => CHINA_AUTH_URI,
            _ => COMMON_AUTH_URI,
        }
    }
}

pub async fn authenticate(
    client_id: String,
    client_secret: String,
    region: Region,
) -> Result<Credentials> {
    let client = reqwest::ClientBuilder::default()
        .build()
        .map_err(|err| Error::TransportError(err))?;

    client
        .get(region.get_auth_uri())
        .basic_auth(client_id, Some(client_secret))
        .send()
        .await
        .map_err(|err| Error::TransportError(err))?;
    Ok(Credentials {
        access_token: "".to_string(),
        expires_in: 0,
        token_type: "".to_string(),
        scope: "".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use httpmock::{prelude::*, server::HttpMockServerBuilder};
    #[tokio::test]
    async fn my_test() {
        let server = HttpMockServerBuilder::
        let hello_mock = server.mock(|when, then| {
            when.method("GET")
                .path("/translate")
                .query_param("word", "hello");
            then.status(200)
                .header("content-type", "text/html; charset=UTF-8")
                .body("hola");
        });
    }
}
