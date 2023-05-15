use sea_query::{Expr, MySqlQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::{AnyPool, Error};

use super::models::account::{Account, AccountIden, NewAccount};

pub async fn create_account(
    account: NewAccount,
    connection_pool: &AnyPool,
) -> Result<Account, Error> {
    let (query, binds) = Query::insert()
        .into_table(AccountIden::Table)
        .columns(vec![AccountIden::Owner, AccountIden::Balance])
        .values_panic(vec![account.owner.into(), account.balance.into()])
        .build_any_sqlx(&MySqlQueryBuilder {});

    let id = sqlx::query_with(&query, binds)
        .execute(connection_pool)
        .await?
        .last_insert_id();

    match id {
        // load account explicitly as mysql does not support INSERT RETURNING statements
        Some(id) => get_account_by_id(id as i32, connection_pool).await,
        None => Err(Error::RowNotFound),
    }
}

pub async fn get_accounts(connection_pool: &AnyPool) -> Result<Vec<Account>, Error> {
    let (query, binds) = Query::select()
        .expr(Expr::asterisk())
        .from(AccountIden::Table)
        .build_any_sqlx(&MySqlQueryBuilder {});

    sqlx::query_as_with::<_, Account, _>(&query, binds)
        .fetch_all(connection_pool)
        .await
}

pub async fn get_account_by_id(id: i32, connection_pool: &AnyPool) -> Result<Account, Error> {
    let (query, binds) = Query::select()
        .expr(Expr::asterisk())
        .from(AccountIden::Table)
        .and_where(Expr::col(AccountIden::Id).eq(id))
        .build_any_sqlx(&MySqlQueryBuilder {});

    sqlx::query_as_with::<_, Account, _>(&query, binds)
        .fetch_one(connection_pool)
        .await
}

pub async fn add_amount(id: i32, amount: f64, connection_pool: &AnyPool) -> Result<Account, Error> {
    let (query, binds) = Query::update()
        .table(AccountIden::Table)
        .values(vec![(
            AccountIden::Balance,
            Expr::col(AccountIden::Balance).add(amount),
        )])
        .and_where(Expr::col(AccountIden::Id).eq(id))
        .build_any_sqlx(&MySqlQueryBuilder {});

    sqlx::query_with(&query, binds)
        .execute(connection_pool)
        .await?;

    get_account_by_id(id, connection_pool).await
}

pub async fn delete_account_by_id(id: i32, connection_pool: &AnyPool) -> Result<i32, Error> {
    let (query, binds) = Query::delete()
        .from_table(AccountIden::Table)
        .and_where(Expr::col(AccountIden::Id).eq(id))
        .build_any_sqlx(&MySqlQueryBuilder {});

    sqlx::query_with(&query, binds)
        .execute(connection_pool)
        .await?;

    Ok(id)
}
