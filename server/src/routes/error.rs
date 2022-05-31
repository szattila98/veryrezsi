use serde::Serialize;

use crate::logic::error::UserError;

#[derive(Debug, Serialize)]
pub struct ErrorMsg {
    reason: String,
}

impl ErrorMsg {
    pub fn new<S: AsRef<str>>(reason: S) -> Self {
        Self {
            reason: reason.as_ref().into(),
        }
    }
}

impl From<UserError> for ErrorMsg {
    fn from(error: UserError) -> Self {
        Self::new(error.to_string())
    }
}
