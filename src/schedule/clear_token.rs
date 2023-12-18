use redis::{AsyncCommands, Client};
use sea_orm::DatabaseConnection;
use sea_orm::*;
use tokio::time::{interval, Duration};

use crate::{entity::sessions, service::token::Token};

pub fn clear_token_timer(redis: Client, db_conn: DatabaseConnection) {
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(20)); // 创建一个20秒的间隔定时器
        loop {
            interval.tick().await;
            // 在这里执行定时任务的操作
            let mut conn = redis.get_async_connection().await.unwrap();
            let now = chrono::Local::now().timestamp();
            let keys: Vec<String> = conn.keys("token:*").await.unwrap();
            for key in keys.iter() {
                let token: Token = conn.get::<_, Token>(key).await.unwrap();
                // 对断线五分钟的用户认为已经下线
                if now - token.updated_at > 60 * 1 {
                    match conn.del::<_, ()>(key).await {
                        Ok(_) => {
                            dbg!("删除成功");
                        }
                        Err(e) => {
                            dbg!(e);
                        }
                    }
                    let db_key = key.replace("token:", "");
                    match sessions::Entity::find_by_id(db_key).one(&db_conn).await {
                        Ok(Some(session)) => {
                            println!("into Some");
                            let mut next_session: sessions::ActiveModel = session.into();
                            next_session.leaved_at = Set(Some(token.updated_at as i32));
                            let _ = next_session.update(&db_conn).await;
                        }

                        Ok(None) => {
                            println!("into None");
                        }
                        Err(e) => {
                            dbg!(e);
                        }
                    }
                }
                dbg!(token);
            }
        }
    });
}
