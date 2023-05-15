use sea_query::enum_def;
use sqlx::FromRow;

#[enum_def]
#[derive(Debug, Clone, FromRow)]
pub struct Account {
    pub id: i32,
    pub owner: String,
    pub balance: f64,
}

pub struct NewAccount {
    pub owner: String,
    pub balance: f64,
}
