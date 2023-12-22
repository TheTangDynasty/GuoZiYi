use std::error::Error;

use crate::utils::app_state::AppState;
use ::entity::sessions;
use chrono::Local;
use redis::Commands;
use redis_macros::{FromRedisValue, ToRedisArgs};
use sea_orm::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRedisValue, ToRedisArgs, Debug)]
pub struct Token {
    pub username: String,
    pub created_at: i64,
    pub updated_at: i64,
}

pub fn get_token_key(uuid: String) -> String {
    format!("token:{}", uuid)
}

impl AppState {
    pub async fn create_token(&mut self, username: String) -> Result<String, Box<dyn Error>> {
        let uuid = Uuid::new_v4().to_string();
        let mut redis_con = self.redis.get_connection()?;
        let now = Local::now().timestamp();
        let _: () = redis_con.set(
            get_token_key(uuid.clone()),
            Token {
                username: username.clone(),
                created_at: now,
                updated_at: now,
            },
        )?;
        sessions::ActiveModel {
            id: Set(uuid.clone()),
            user_name: Set(username.clone()),
            created_at: Set(now),
            leaved_at: Set(now),
            ..Default::default()
        }
        .insert(&self.db)
        .await?;

        Ok(uuid.clone())
    }

    pub async fn update_token(&mut self, uuid: String) -> redis::RedisResult<()> {
        let mut con = self.redis.get_connection()?;
        let now = Local::now().timestamp();
        let token_key = get_token_key(uuid.clone());
        let old_cache = con.get::<_, Token>(token_key.clone())?;

        let _: () = con.set(
            token_key,
            Token {
                username: old_cache.username,
                created_at: old_cache.created_at,
                updated_at: now,
            },
        )?;
        Ok(())
    }

    pub async fn get_alive_tokens(&mut self) -> Vec<Token> {
        let mut con = self.redis.get_connection().unwrap();
        let keys: Vec<String> = con.keys("token:*").unwrap();
        let mut tokens: Vec<Token> = vec![];
        for key in keys {
            match con.get::<_, Token>(key) {
                Ok(token) => {
                    tokens.push(token);
                }
                Err(_) => {
                    continue;
                }
            }
        }
        tokens.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        tokens
    }

    pub async fn get_token(
        &mut self,
        uuid: String,
    ) -> Result<Option<sessions::Model>, sea_orm::DbErr> {
        sessions::Entity::find_by_id(uuid.clone())
            .one(&self.db)
            .await
    }
}
