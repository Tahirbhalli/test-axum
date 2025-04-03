use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_column(date_time(User::UpdatedAt).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
 
        manager
        .alter_table(
            Table::alter()
                .table(User::Table)
                .drop_column(User::UpdatedAt)
                .to_owned(),
        )
        .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    UpdatedAt,
}
