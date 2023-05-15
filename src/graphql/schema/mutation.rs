use actix_web_lab::__reexports::tracing::log::warn;
use juniper::{FieldResult, IntoFieldError};

use crate::{
    graphql::{graphql_context::GraphQlContext, schema::query::Account},
    service::account_service,
};

pub struct Mutation;
#[juniper::graphql_object(Context = GraphQlContext)]
impl Mutation {
    pub async fn createAccount(context: &GraphQlContext, owner: String) -> FieldResult<Account> {
        let account = account_service::create_account_for(owner, &context.connection_pool).await;

        match account {
            Ok(account) => FieldResult::Ok(account.into()),
            Err(e) => {
                warn!("Failed to create account: {}", e);
                FieldResult::Err(e.into_field_error())
            }
        }
    }

    pub async fn deposit(
        context: &GraphQlContext,
        account_number: i32,
        amount: f64,
    ) -> FieldResult<Account> {
        let account =
            account_service::deposit(account_number, amount, &context.connection_pool).await;

        match account {
            Ok(account) => FieldResult::Ok(account.into()),
            Err(e) => {
                warn!("Failed to deposit: {}", e);
                FieldResult::Err(e.into_field_error())
            }
        }
    }

    pub async fn withdraw(
        context: &GraphQlContext,
        account_number: i32,
        amount: f64,
    ) -> FieldResult<Account> {
        let account =
            account_service::withdraw(account_number, amount, &context.connection_pool).await;

        match account {
            Ok(account) => FieldResult::Ok(account.into()),
            Err(e) => {
                warn!("Failed to withdraw: {}", e);
                FieldResult::Err(e.into_field_error())
            }
        }
    }

    pub async fn transfer(
        context: &GraphQlContext,
        from_account_number: i32,
        to_account_number: i32,
        amount: f64,
    ) -> FieldResult<Account> {
        let account = account_service::transfer(
            from_account_number,
            to_account_number,
            amount,
            &context.connection_pool,
        )
        .await;

        match account {
            Ok(account) => FieldResult::Ok(account.into()),
            Err(e) => {
                warn!("Failed to transfer: {}", e);
                FieldResult::Err(e.into_field_error())
            }
        }
    }

    pub async fn close_account(context: &GraphQlContext, account_number: i32) -> FieldResult<bool> {
        let id = account_service::close_account(account_number, &context.connection_pool).await;

        match id {
            Ok(_) => FieldResult::Ok(true),
            Err(e) => {
                warn!("Failed to close account: {}", e);
                FieldResult::Err(e.into_field_error())
            }
        }
    }
}
