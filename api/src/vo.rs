use entity::users;
use sea_orm::entity::prelude::DateTime;
use serde::Serialize;

#[derive(Serialize)]
pub struct UserVo {
    pub id: i32,
    pub email: String,
    pub nickname: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl From<users::Model> for UserVo {
    fn from(value: users::Model) -> Self {
        Self {
            id: value.id,
            email: value.email,
            nickname: value.nickname,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
