use ::entity::{prelude::*, session};
use sea_orm::*;

pub struct SessionRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> SessionRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_by_join_code(
        &self,
        join_code: String,
    ) -> Result<Option<session::Model>, DbErr> {
        Session::find()
            .filter(session::Column::JoinCode.eq(join_code))
            .one(self.db)
            .await
    }

    pub async fn create(&self, session: session::ActiveModel) -> Result<session::Model, DbErr> {
        session.insert(self.db).await
    }
}
