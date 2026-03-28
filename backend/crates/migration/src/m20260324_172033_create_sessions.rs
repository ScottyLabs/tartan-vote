use crate::m20260308_183617_create_users::User;
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
                    .as_enum(Status::Enum)
                    .values([Status::Open, Status::Locked, Status::Closed])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Session::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Session::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Session::JoinCode).string().not_null())
                    .col(
                        ColumnDef::new(Session::Status)
                            .custom(Status::Enum)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Session::CreatedByUserId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Session::Table, Session::CreatedByUserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Session::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(Status::Enum).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Session {
    Table,
    Id,
    JoinCode,
    Status,
    CreatedByUserId,
}

#[derive(DeriveIden)]
pub enum Status {
    #[sea_orm(iden = "session_status")]
    Enum,
    Open,
    Locked,
    Closed,
}
