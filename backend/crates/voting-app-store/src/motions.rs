use ::entity::{enums::StatusOption, motion, prelude::*};
use sea_orm::*;

pub struct MotionRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> MotionRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<motion::Model>, DbErr> {
        Motion::find_by_id(id).one(self.db).await
    }

    pub async fn create(&self, motion: motion::ActiveModel) -> Result<motion::Model, DbErr> {
        motion.insert(self.db).await
    }

    pub async fn update(&self, motion: motion::ActiveModel) -> Result<motion::Model, DbErr> {
        motion.update(self.db).await
    }

    pub async fn delete(&self, id: i32) -> Result<DeleteResult, DbErr> {
        Motion::delete_by_id(id).exec(self.db).await
    }

    pub async fn find_active_by_session_id(
        &self,
        session_id: i32,
    ) -> Result<Option<motion::Model>, DbErr> {
        Motion::find()
            .filter(motion::Column::SessionId.eq(session_id))
            .filter(motion::Column::Status.eq(StatusOption::Active))
            .one(self.db)
            .await
    }
}
