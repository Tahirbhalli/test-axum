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
                    .table(Post::Table)
                    .add_column(integer(Post::UserId).not_null())
                    .add_foreign_key(
                        &TableForeignKey::new()
                        .name("fk_post_user")
                        .from_tbl(Post::Table)
                        .from_col(Post::UserId)
                        .to_tbl(User::Table)
                        .to_col(User::Id)
                        .on_delete(ForeignKeyAction::Cascade)
                        .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
        .alter_table(
            Table::alter()
                .table(Post::Table)
                .drop_column(Post::UserId)
                .to_owned(),
        )
        .await
    }
}

#[derive(DeriveIden)]
enum User{
    Table,
    Id
}


#[derive(DeriveIden)]
enum Post {
    Table,
    UserId
}
