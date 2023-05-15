use std::fmt::{Display, Formatter};

pub enum AccountError {
    AccountNotFound,
    InsufficientFunds,
    CannotCloseAccount,
    InternalServerError,
}
impl Display for AccountError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountError::AccountNotFound => write!(f, "Account not found"),
            AccountError::InsufficientFunds => write!(f, "Insufficient funds"),
            AccountError::CannotCloseAccount => write!(f, "Cannot close account"),
            AccountError::InternalServerError => write!(f, "Internal server error"),
        }
    }
}
