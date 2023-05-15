use actix_web_lab::__reexports::tracing::log::warn;
use sqlx::AnyPool;

use crate::{
    error::AccountError,
    persistence::{
        account_repository,
        models::account::{Account, NewAccount},
    },
};

pub async fn create_account_for(
    owner: String,
    connection_pool: &AnyPool,
) -> Result<Account, AccountError> {
    let account = NewAccount {
        owner,
        balance: 0.0,
    };

    convert_db_result(account_repository::create_account(account, connection_pool).await)
}

pub async fn get_accounts(connection_pool: &AnyPool) -> Result<Vec<Account>, AccountError> {
    convert_db_result(account_repository::get_accounts(connection_pool).await)
}

pub async fn get_account_by_id(
    account_number: i32,
    connection_pool: &AnyPool,
) -> Result<Account, AccountError> {
    convert_db_result(account_repository::get_account_by_id(account_number, connection_pool).await)
}

pub async fn withdraw(
    account_number: i32,
    amount: f64,
    connection_pool: &AnyPool,
) -> Result<Account, AccountError> {
    match account_repository::get_account_by_id(account_number, connection_pool).await {
        Ok(account) => {
            if account.balance < amount {
                return Err(AccountError::InsufficientFunds);
            }

            convert_db_result(
                account_repository::add_amount(account_number, -amount, connection_pool).await,
            )
        }
        Err(e) => Err(convert_sql_error(e)),
    }
}

pub async fn deposit(
    account_number: i32,
    amount: f64,
    connection_pool: &AnyPool,
) -> Result<Account, AccountError> {
    convert_db_result(account_repository::add_amount(account_number, amount, connection_pool).await)
}

pub async fn transfer(
    from_account_number: i32,
    to_account_number: i32,
    amount: f64,
    connection_pool: &AnyPool,
) -> Result<Account, AccountError> {
    let transaction = connection_pool.begin().await.unwrap();

    let from = convert_db_result(
        account_repository::add_amount(from_account_number, -amount, connection_pool).await,
    )?;
    convert_db_result(
        account_repository::add_amount(to_account_number, amount, connection_pool).await,
    )?;

    transaction.commit().await.unwrap();
    Ok(from)
}

pub async fn close_account(
    account_number: i32,
    connection_pool: &AnyPool,
) -> Result<i32, AccountError> {
    match account_repository::get_account_by_id(account_number, connection_pool).await {
        Ok(account) => {
            if account.balance > 0.0 {
                return Err(AccountError::CannotCloseAccount);
            }

            convert_db_result(
                account_repository::delete_account_by_id(account_number, connection_pool).await,
            )
        }
        Err(e) => Err(convert_sql_error(e)),
    }
}

fn convert_db_result<T>(res: Result<T, sqlx::Error>) -> Result<T, AccountError> {
    match res {
        Ok(val) => Ok(val),
        Err(e) => Err(convert_sql_error(e)),
    }
}

fn convert_sql_error(res: sqlx::Error) -> AccountError {
    match res {
        sqlx::Error::RowNotFound => AccountError::AccountNotFound,
        _ => {
            warn!("sqlx error: {}", res);
            AccountError::InternalServerError
        }
    }
}
