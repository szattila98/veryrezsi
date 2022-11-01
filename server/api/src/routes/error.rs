use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use validator::ValidationErrors;
use veryrezsi_core::logic::error::{
    CurrencyError, ExpenseError, ExpenseTransactionError, RecurrenceError, UserError,
};

/// A struct that can be returned from route handlers on error.
/// It has an optional generic details parameter, which is used to return more detailed information about the error (e.g. validation errors).
/// If none, it won't be serialized.
/// ```
/// use veryrezsi::routes::error::ErrorMsg;
/// use axum::http::StatusCode;
/// use validator::ValidationErrors;
///
/// let msg: ErrorMsg<ValidationErrors> = ErrorMsg::new(StatusCode::BAD_REQUEST, "invalid username")
///     .details(ValidationErrors::new());
/// ```
/// On empty details use `()` as the generic parameter.
/// ```
/// use veryrezsi::routes::error::ErrorMsg;
/// use axum::http::StatusCode;
/// use validator::ValidationErrors;
///
/// let msg: ErrorMsg<()> = ErrorMsg::new(StatusCode::BAD_REQUEST, "invalid username");
/// ```
#[derive(Debug, Serialize)]
pub struct ErrorMsg<D: Serialize> {
    #[serde(skip_serializing)]
    status: StatusCode,
    reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<D>, // Option is needed until specialization feature is stable, then we can use a trait to test whether D is a type or ()
}

impl<D: Serialize> ErrorMsg<D> {
    /// Creates a new ErrorMsg with the given status code and reason, without details.
    /// Reason is generic over any string-like type.
    pub fn new<S: AsRef<str>>(status: StatusCode, reason: S) -> Self {
        Self {
            status,
            reason: reason.as_ref().into(),
            details: None,
        }
    }

    /// Builder function, so details field in constructor is optional.
    pub fn details(mut self, details: D) -> Self {
        self.details = Some(details);
        self
    }
}

impl<D: Serialize> IntoResponse for ErrorMsg<D> {
    fn into_response(self) -> Response {
        (self.status, Json(self)).into_response()
    }
}

impl<D: Serialize> From<JsonRejection> for ErrorMsg<D> {
    fn from(e: JsonRejection) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    }
}

impl From<ValidationErrors> for ErrorMsg<ValidationErrors> {
    fn from(e: ValidationErrors) -> Self {
        Self::new(StatusCode::BAD_REQUEST, "validation of inputs failed").details(e)
    }
}

impl<D: Serialize> From<UserError> for ErrorMsg<D> {
    fn from(e: UserError) -> Self {
        match e {
            UserError::UserNotFound(_) => Self::new(StatusCode::NOT_FOUND, e.to_string()),
            UserError::EmailAlreadyExists(_) => Self::new(StatusCode::BAD_REQUEST, e.to_string()),
            UserError::PasswordCannotBeHashed(_) => {
                Self::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
            UserError::ActivationTokenNotFound(_) => {
                Self::new(StatusCode::BAD_REQUEST, e.to_string())
            }
            UserError::ActivationTokenExpired => Self::new(StatusCode::BAD_REQUEST, e.to_string()),
            UserError::UserHasNoRightForAction => Self::new(StatusCode::FORBIDDEN, e.to_string()),
            UserError::DatabaseError(_) => {
                Self::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
    }
}

impl<D: Serialize> From<ExpenseError> for ErrorMsg<D> {
    fn from(e: ExpenseError) -> Self {
        match e {
            ExpenseError::InvalidExpenseData(_) => {
                Self::new(StatusCode::BAD_REQUEST, e.to_string())
            }
            ExpenseError::DatabaseError(_) => {
                Self::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
    }
}

impl<D: Serialize> From<ExpenseTransactionError> for ErrorMsg<D> {
    fn from(e: ExpenseTransactionError) -> Self {
        match e {
            ExpenseTransactionError::InvalidTransactionData(_) => {
                Self::new(StatusCode::BAD_REQUEST, e.to_string())
            }
            ExpenseTransactionError::DatabaseError(_) => {
                Self::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
            ExpenseTransactionError::TransactionToDeletedDoesNotExist => {
                Self::new(StatusCode::NO_CONTENT, e.to_string())
            }
            ExpenseTransactionError::ParentExpenseIsNotOwnedByTheUser(_) => {
                Self::new(StatusCode::FORBIDDEN, e.to_string())
            }
        }
    }
}

impl<D: Serialize> From<CurrencyError> for ErrorMsg<D> {
    fn from(e: CurrencyError) -> Self {
        match e {
            CurrencyError::DatabaseError(_) => {
                Self::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
    }
}

impl<D: Serialize> From<RecurrenceError> for ErrorMsg<D> {
    fn from(e: RecurrenceError) -> Self {
        match e {
            RecurrenceError::DatabaseError(_) => {
                Self::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
    }
}
