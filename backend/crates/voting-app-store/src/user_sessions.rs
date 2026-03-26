use ::entity::{prelude::*, user_session};
use sea_orm::*;

pub struct UserSessionRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> UserSessionRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    pub async fn fetch_by_session_id(
        &self,
        session_id: i32,
    ) -> Result<Vec<user_session::Model>, DbErr> {
        UserSession::find()
            .filter(user_session::Column::SessionId.eq(session_id))
            .all(self.db)
            .await
    }

    pub async fn fetch_by_user_id(&self, user_id: i32) -> Result<Vec<user_session::Model>, DbErr> {
        UserSession::find()
            .filter(user_session::Column::UserId.eq(user_id))
            .all(self.db)
            .await
    }

    pub async fn create(
        &self,
        user_session: user_session::ActiveModel,
    ) -> Result<user_session::Model, DbErr> {
        user_session.insert(self.db).await
    }
}
