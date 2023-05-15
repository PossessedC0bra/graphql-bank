use juniper::{FieldResult, GraphQLObject, IntoFieldError};

use crate::{
    graphql::graphql_context::GraphQlContext,
    persistence::models::account::Account as AccountEntity, service::account_service,
};

pub struct Query;
#[juniper::graphql_object(Context = GraphQlContext)]
impl Query {
    pub async fn accounts(context: &GraphQlContext) -> FieldResult<Vec<self::Account>> {
        let accounts = account_service::get_accounts(&context.connection_pool).await;

        match accounts {
            Ok(accounts) => FieldResult::Ok(accounts.into_iter().map(|acc| acc.into()).collect()),
            Err(e) => FieldResult::Err(e.into_field_error()),
        }
    }

    pub async fn account(
        context: &GraphQlContext,
        account_number: i32,
    ) -> FieldResult<self::Account> {
        let account =
            account_service::get_account_by_id(account_number, &context.connection_pool).await;

        match account {
            Ok(account) => FieldResult::Ok(account.into()),
            Err(e) => FieldResult::Err(e.into_field_error()),
        }
    }
}

#[derive(Default, Debug, GraphQLObject)]
pub struct Account {
    pub account_number: i32,
    pub owner: String,
    pub balance: f64,
}
impl Into<Account> for AccountEntity {
    fn into(self) -> Account {
        Account {
            account_number: self.id,
            owner: self.owner,
            balance: self.balance,
        }
    }
}
