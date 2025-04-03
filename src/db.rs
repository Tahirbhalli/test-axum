use sea_orm::{Database, DatabaseConnection};

/// Establishes a connection to the PostgreSQL database
///
/// # Arguments
/// * `database_url` - The database connection URL (e.g., "postgres://user:password@localhost:5432/db_name")
///
/// # Returns
/// * `Ok(DatabaseConnection)` on successful connection
/// * `Err(DbErr)` if connection fails
pub async fn establish_connection(url: &str) -> Result<DatabaseConnection, sea_orm::DbErr> {
    let db = Database::connect(url).await?.into();

    Ok(db)
}
