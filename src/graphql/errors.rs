use juniper::{graphql_value, IntoFieldError};

use crate::error::AccountError;

impl IntoFieldError for AccountError {
    fn into_field_error(self) -> juniper::FieldError {
        match self {
            AccountError::AccountNotFound => juniper::FieldError::new(
                "Account not found",
                graphql_value!({
                    "code": "ACCOUNT_NOT_FOUND_ERROR"
                }),
            ),
            AccountError::InsufficientFunds => juniper::FieldError::new(
                "Insufficient funds",
                graphql_value!({
                    "code": "INSUFFICIENT_FUNDS_ERROR"
                }),
            ),
            AccountError::CannotCloseAccount => juniper::FieldError::new(
                "Cannot close account",
                graphql_value!({
                    "code": "CANNOT_CLOSE_ACCOUNT_ERROR"
                }),
            ),
            AccountError::InternalServerError => juniper::FieldError::new(
                "Internal server error",
                graphql_value!({
                    "code": "INTERNAL_SERVER_ERROR"
                }),
            ),
        }
    }
}
