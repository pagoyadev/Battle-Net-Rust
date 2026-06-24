#![forbid(unsafe_code)]

extern crate thiserror;
use thiserror::Error;

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

#[derive(Error, Debug)]
enum Error {
    #[error("Transport: {0}")]
    TransportError(reqwest::Error),
}

pub fn authenticate(client_id: String, client_secret: String, region: Region) {
    let client = reqwest::ClientBuilder::default()
        .build()
        .map_err(|err| Error::TransportError(err));

    client
        .get(region.get_auth_uri())
        .basic_auth(client_id, Some(client_secret))
        .send();
}
