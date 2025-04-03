pub use sea_orm_migration::prelude::*;

mod m20250331_044411_create_users_table;
mod m20250331_083103_add_column_to_users;
mod m20250331_085326_add_posts;
mod m20250331_085657_add_user_relation_to_posts;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250331_044411_create_users_table::Migration),
            Box::new(m20250331_083103_add_column_to_users::Migration),
            Box::new(m20250331_085326_add_posts::Migration),
            Box::new(m20250331_085657_add_user_relation_to_posts::Migration),
        ]
    }
}
