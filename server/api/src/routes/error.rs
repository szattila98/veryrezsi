use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use migration::DbErr;
use serde::Serialize;
use validator::ValidationErrors;
use veryrezsi_core::logic::{
    expense_operations::errors::{
        CreateExpenseError, CreatePredefinedExpenseError, FindExpensesWithTransactionsByUserIdError,
    },
    transaction_operations::errors::{CreateTransactionError, DeleteTransactionByIdError},
    user_operations::errors::{ActivateAccountError, SaveUserError, VerifyLoginError},
};

/// A struct that can be returned from route handlers on error.
/// It has an optional generic details parameter, which is used to return more detailed information about the error (e.g. validation errors).
/// If none, it won't be serialized.
/// ```
/// use veryrezsi_api::routes::error::ErrorMsg;
/// use axum::http::StatusCode;
/// use validator::ValidationErrors;
///
/// let msg: ErrorMsg<ValidationErrors> = ErrorMsg::new(StatusCode::BAD_REQUEST, "invalid username")
///     .details(ValidationErrors::new());
/// ```
/// On empty details use `()` as the generic parameter.
/// ```
/// use veryrezsi_api::routes::error::ErrorMsg;
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
    /// Creates a new `ErrorMsg` with the given status code and reason, without details.
    /// Reason is generic over any string-like type.
    pub fn new<S: AsRef<str>>(status: StatusCode, reason: S) -> Self {
        Self {
            status,
            reason: reason.as_ref().into(),
            details: None,
        }
    }

    /// Builder function, so details field in constructor is optional.
    #[must_use]
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

impl<D: Serialize> From<DbErr> for ErrorMsg<D> {
    fn from(e: DbErr) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    }
}

impl<D: Serialize> From<VerifyLoginError> for ErrorMsg<D> {
    fn from(e: VerifyLoginError) -> Self {
        match e {
            VerifyLoginError::AccountNotActivated => {
                Self::new(StatusCode::BAD_REQUEST, e.to_string())
            }
            VerifyLoginError::IncorrectCredentials => {
                Self::new(StatusCode::UNAUTHORIZED, e.to_string())
            }
            VerifyLoginError::DatabaseError(db_error) => db_error.into(),
        }
    }
}

impl<D: Serialize> From<SaveUserError> for ErrorMsg<D> {
    fn from(e: SaveUserError) -> Self {
        match e {
            SaveUserError::UserAlreadyExists => Self::new(StatusCode::BAD_REQUEST, e.to_string()),
            SaveUserError::PasswordCannotBeHashed(msg) => {
                Self::new(StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
            SaveUserError::EmailCannotBeSent(reason) => {
                Self::new(StatusCode::INTERNAL_SERVER_ERROR, reason)
            }
            SaveUserError::DatabaseError(db_error) => db_error.into(),
        }
    }
}

impl<D: Serialize> From<ActivateAccountError> for ErrorMsg<D> {
    fn from(e: ActivateAccountError) -> Self {
        match e {
            ActivateAccountError::InvalidToken => Self::new(StatusCode::BAD_REQUEST, e.to_string()),
            ActivateAccountError::DatabaseError(db_error) => db_error.into(),
        }
    }
}

impl<D: Serialize> From<FindExpensesWithTransactionsByUserIdError> for ErrorMsg<D> {
    fn from(e: FindExpensesWithTransactionsByUserIdError) -> Self {
        match e {
            FindExpensesWithTransactionsByUserIdError::UnauthorizedUser(_) => {
                Self::new(StatusCode::FORBIDDEN, e.to_string())
            }
            FindExpensesWithTransactionsByUserIdError::DatabaseError(db_error) => db_error.into(),
        }
    }
}

impl<D: Serialize> From<CreateExpenseError> for ErrorMsg<D> {
    fn from(e: CreateExpenseError) -> Self {
        match e {
            CreateExpenseError::InvalidPredefinedExpense
            | CreateExpenseError::InvalidRelatedType(_) => {
                Self::new(StatusCode::NOT_FOUND, e.to_string())
            }
            CreateExpenseError::InvalidStartDate(_) => {
                Self::new(StatusCode::BAD_REQUEST, e.to_string())
            }
            CreateExpenseError::DatabaseError(db_error) => db_error.into(),
        }
    }
}

impl<D: Serialize> From<CreatePredefinedExpenseError> for ErrorMsg<D> {
    fn from(e: CreatePredefinedExpenseError) -> Self {
        match e {
            CreatePredefinedExpenseError::InvalidRelatedType(_) => {
                Self::new(StatusCode::NOT_FOUND, e.to_string())
            }
            CreatePredefinedExpenseError::DatabaseError(db_error) => db_error.into(),
        }
    }
}

impl<D: Serialize> From<CreateTransactionError> for ErrorMsg<D> {
    fn from(e: CreateTransactionError) -> Self {
        match e {
            CreateTransactionError::InvalidExpenseId | CreateTransactionError::InvalidCurrency => {
                Self::new(StatusCode::NOT_FOUND, e.to_string())
            }
            CreateTransactionError::UserUnauthorized(_) => {
                Self::new(StatusCode::UNAUTHORIZED, e.to_string())
            }
            CreateTransactionError::InvalidStartDate(_) => {
                Self::new(StatusCode::BAD_REQUEST, e.to_string())
            }
            CreateTransactionError::DatabaseError(db_error) => db_error.into(),
        }
    }
}

impl<D: Serialize> From<DeleteTransactionByIdError> for ErrorMsg<D> {
    fn from(e: DeleteTransactionByIdError) -> Self {
        match e {
            DeleteTransactionByIdError::InvalidTransaction => {
                Self::new(StatusCode::NOT_FOUND, e.to_string())
            }
            DeleteTransactionByIdError::UserUnauthorized(_) => {
                Self::new(StatusCode::UNAUTHORIZED, e.to_string())
            }
            DeleteTransactionByIdError::DatabaseError(db_error) => db_error.into(),
        }
    }
}
