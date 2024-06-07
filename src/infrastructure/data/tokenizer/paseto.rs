use rusty_paseto::prelude::*;
use serde_json::Value;
use time::format_description::well_known::Rfc3339;

use crate::infrastructure::{data::context::config::AppConfig, errors::tokenizer_error::TokenizerError};

#[derive(Debug, Clone)]
pub struct PasetoMaker {
    cfg: AppConfig,
}

impl PasetoMaker {
    pub fn new(cfg: AppConfig) -> Self {
        Self { cfg }
    }

    pub fn create_token(&self, user_id: &String, role: &String) -> Result<String, TokenizerError> {
        let secret = self.cfg.secret_key.as_bytes();
        let key = PasetoSymmetricKey::<V4, Local>::from(Key::from(secret));
        let user_id_str: &str = user_id.as_str();
        let in_1_week = (time::OffsetDateTime::now_utc() + time::Duration::hours(168)).format(&Rfc3339)?;

        let token = PasetoBuilder::<V4, Local>::default()
            .set_claim(ExpirationClaim::try_from(in_1_week)?)
            .set_claim(SubjectClaim::from(user_id_str))
            .set_claim(AudienceClaim::from(role.as_str()))
            .build(&key)?;

        Ok(token)
    }

    pub fn verify_token(&self, token: String) -> Result<Value, TokenizerError> {
        let secret = self.cfg.secret_key.as_bytes();
        let key = PasetoSymmetricKey::<V4, Local>::from(Key::from(secret));
        let json_value = PasetoParser::<V4, Local>::default().parse(&token, &key)?;

        Ok(json_value)
    }
}