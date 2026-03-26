use crate::m20260308_183617_create_users::User;
use crate::m20260324_172033_create_sessions::Session;
use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(JoinLeft::Enum)
                    .values([JoinLeft::Joined, JoinLeft::Left])
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(UserSession::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserSession::UserId).integer().not_null())
                    .col(ColumnDef::new(UserSession::SessionId).integer().not_null())
                    .col(
                        ColumnDef::new(UserSession::JoinLeft)
                            .custom(JoinLeft::Enum)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserSession::Timestamp)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserSession::Table, UserSession::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserSession::Table, UserSession::SessionId)
                            .to(Session::Table, Session::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserSession::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(JoinLeft::Enum).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum UserSession {
    Table,
    UserId,
    SessionId,
    JoinLeft,
    Timestamp,
}

#[derive(DeriveIden)]
pub enum JoinLeft {
    #[sea_orm(iden = "join_left")]
    Enum,
    Joined,
    Left,
}
