use sqlx::AnyPool;
use std::env;

pub async fn create_connection_pool() -> AnyPool {
    AnyPool::connect(
        &env::var("DATABASE_URL").expect("Failed to read environment variable DATABASE_URL"),
    )
    .await
    .expect("Failed to bulid connection pool")
}
