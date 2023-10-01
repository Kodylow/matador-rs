use macaroon::MacaroonKey;

use crate::{Error, Result};
use serde_json::Value;
use std::env;
use std::process::Command;
use std::str::FromStr;
use std::{collections::HashMap, sync::OnceLock};
use time::OffsetDateTime;

use super::{get_env, get_env_b64u_as_u8s, get_env_parse, get_env_parse_to_macaroon_key};

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct Config {
    // -- Crypt
    pub PWD_KEY: Vec<u8>,
    pub TOKEN_KEY: Vec<u8>,
    pub TOKEN_DURATION_SEC: f64,
    pub MACAROON_KEY: MacaroonKey,

    // -- Db
    pub DB_URL: String,

    // -- Web
    pub WEB_FOLDER: String,

    // -- Lightning
    pub LIGHTNING_ADDRESS: String,
}

impl Config {
    fn load_from_env() -> Result<Config> {
        Ok(Config {
            // -- Crypt
            PWD_KEY: get_env_b64u_as_u8s("SERVICE_PWD_KEY")?,

            TOKEN_KEY: get_env_b64u_as_u8s("SERVICE_TOKEN_KEY")?,
            TOKEN_DURATION_SEC: get_env_parse("SERVICE_TOKEN_DURATION_SEC")?,
            MACAROON_KEY: get_env_parse_to_macaroon_key("SERVICE_MACAROON_KEY")?,

            // -- Db
            DB_URL: get_env("SERVICE_DB_URL")?,

            // -- Web
            WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,

            // -- Lightning
            LIGHTNING_ADDRESS: get_env("SERVICE_LIGHTNING_ADDRESS")?,
        })
    }
}