use ::entity::{enums::StatusOption, event, prelude::*};
use sea_orm::*;

pub struct EventRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> EventRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<event::Model>, DbErr> {
        Event::find_by_id(id).one(self.db).await
    }

    pub async fn create(&self, event: event::ActiveModel) -> Result<event::Model, DbErr> {
        event.insert(self.db).await
    }

    pub async fn update(&self, event: event::ActiveModel) -> Result<event::Model, DbErr> {
        event.update(self.db).await
    }

    pub async fn delete(&self, id: i32) -> Result<DeleteResult, DbErr> {
        Event::delete_by_id(id).exec(self.db).await
    }

    pub async fn find_active_by_session_id(
        &self,
        session_id: i32,
    ) -> Result<Option<event::Model>, DbErr> {
        Event::find()
            .filter(event::Column::SessionId.eq(session_id))
            .filter(event::Column::Status.eq(StatusOption::Active))
            .one(self.db)
            .await
    }
}
