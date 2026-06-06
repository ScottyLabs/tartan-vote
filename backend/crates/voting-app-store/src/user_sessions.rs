use ::entity::{prelude::*, user, user_session};
use sea_orm::*;

pub struct UserSessionRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> UserSessionRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &DatabaseConnection {
        self.db
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

    pub async fn get_distinct_users(&self, session_id: i32) -> Result<Vec<user::Model>, DbErr> {
        let mut user_ids: Vec<i32> = UserSession::find()
            .filter(user_session::Column::SessionId.eq(session_id))
            .all(self.db)
            .await?
            .into_iter()
            .map(|entry| entry.user_id)
            .collect();

        user_ids.sort_unstable();
        user_ids.dedup();

        if user_ids.is_empty() {
            return Ok(vec![]);
        }

        User::find()
            .filter(user::Column::Id.is_in(user_ids))
            .all(self.db)
            .await
    }
}
