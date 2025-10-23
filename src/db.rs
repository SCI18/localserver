use sqlx::{sqlite::SqlitePool, migrate::MigrateDatabase, Sqlite};

pub async fn init_db() -> Result<SqlitePool, Box<dyn std::error::Error>> {
    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://ide_server.db".to_string());

    // Create database if it doesn't exist
    if !Sqlite::database_exists(&db_url).await? {
        tracing::info!("Creating database: {}", db_url);
        Sqlite::create_database(&db_url).await?;
    }

    let pool = SqlitePool::connect(&db_url).await?;
    
    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    tracing::info!("Database initialized successfully");
    Ok(pool)
}
