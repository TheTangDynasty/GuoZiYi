use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue, DbConn, DbErr, EntityTrait};
use uuid::Uuid;

use crate::entity::sessions;

pub async fn create_session(
    db: &DbConn,
    ip: String,
    user_name: String,
    duration: i64,
) -> Result<sessions::Model, DbErr> {
    let now = Utc::now();
    let ts = now.timestamp() as i32;
    let new_session = sessions::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4().to_string()),
        ip: ActiveValue::Set(Some(ip)),
        created_time: ActiveValue::Set(Some(ts)),
        duration: ActiveValue::Set(Some(duration as i32)),
        user_name: ActiveValue::Set(Some(user_name)),
    };
    let res: sessions::Model = new_session.insert(db).await?;
    Ok(res)
}
