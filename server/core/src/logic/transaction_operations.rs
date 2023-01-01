use super::common;
use super::error::ExpenseTransactionError;
use super::user_operations::authorize_user_by_id;
use crate::dto::transactions::NewTransactionRequest;

use entity::currency_type::{self, Entity as CurrencyType};
use entity::expense::{self, Entity as Expense};
use entity::transaction::{self, Entity as Transaction};
use entity::Id;

use chrono::NaiveDate;
use sea_orm::ActiveValue::NotSet;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub async fn create_transaction(
    conn: &DatabaseConnection,
    user_id: Id,
    req: NewTransactionRequest,
) -> Result<Id, ExpenseTransactionError> {
    let (referred_expense, currency_validation_result) = tokio::join!(
        get_expense_by_id_if_exists(conn, req.expense_id),
        validate_referred_currency_type(conn, req.currency_type_id)
    );

    currency_validation_result?;
    authorize_user_by_id(user_id, referred_expense?.user_id)?;

    let parsed_date = NaiveDate::parse_from_str(&req.date, common::DATE_FORMAT)?;
    let transaction = transaction::ActiveModel {
        id: NotSet,
        donor_name: Set(req.donor_name),
        currency_type_id: Set(req.currency_type_id),
        value: Set(req.value),
        date: Set(parsed_date),
        expense_id: Set(req.expense_id),
    };
    let transaction = transaction.insert(conn).await?;
    Ok(transaction.id)
}

pub async fn delete_transaction_by_id(
    conn: &DatabaseConnection,
    user_id: Id,
    transaction_id: Id,
) -> Result<(), ExpenseTransactionError> {
    let transaction = get_transaction_by_id_if_exists(conn, transaction_id).await?;
    let parent_expense = get_expense_by_id_if_exists(conn, transaction.expense_id).await?;
    authorize_user_by_id(user_id, parent_expense.user_id)?;

    transaction::Entity::delete_by_id(transaction_id)
        .exec(conn)
        .await?;
    Ok(())
}

async fn get_transaction_by_id_if_exists(
    conn: &DatabaseConnection,
    transaction_id: Id,
) -> Result<transaction::Model, ExpenseTransactionError> {
    let transaction = Transaction::find_by_id(transaction_id).one(conn);

    match transaction.await? {
        Some(res) => Ok(res),
        None => Err(ExpenseTransactionError::TransactionToDeleteDoesNotExist),
    }
}

// TODO extract into expense operations and write tests
async fn get_expense_by_id_if_exists(
    conn: &DatabaseConnection,
    expense_id: Id,
) -> Result<expense::Model, ExpenseTransactionError> {
    let expense = Expense::find()
        .filter(expense::Column::Id.eq(expense_id))
        .one(conn);

    match expense.await? {
        Some(res) => Ok(res),
        None => Err(ExpenseTransactionError::InvalidTransactionData(
            String::from("We have not found the referred expense for the transaction"),
        )),
    }
}

// TODO use currency type query instead
async fn validate_referred_currency_type(
    conn: &DatabaseConnection,
    currency_type_id: Id,
) -> Result<(), ExpenseTransactionError> {
    let referred_currency_type = CurrencyType::find()
        .filter(currency_type::Column::Id.eq(currency_type_id))
        .one(conn);
    if referred_currency_type.await?.is_none() {
        return Err(ExpenseTransactionError::InvalidTransactionData(
            String::from("We have no currency type with the given id."),
        ));
    };
    Ok(())
}

mod errors {}

#[cfg(test)]
mod tests {
    use super::*;
    use migration::DbErr;
    use sea_orm::{DatabaseBackend, MockDatabase};

    #[tokio::test]
    async fn create_transaction_on_correct_data_returns_ok() {}

    #[tokio::test]
    async fn create_transaction_on_wrong_expense_id_returns_err() {}

    #[tokio::test]
    async fn create_transaction_on_wrong_currency_id_returns_err() {}

    #[tokio::test]
    async fn create_transaction_on_date_parse_error_returns_err() {}

    #[tokio::test]
    async fn create_transaction_on_dberr_returns_err() {}

    #[tokio::test]
    async fn delete_transaction_on_correct_data_returns_ok() {}

    #[tokio::test]
    async fn delete_transaction_on_wrong_expense_id_returns_err() {}

    #[tokio::test]
    async fn delete_transaction_on_wrong_transaction_id_returns_err() {}

    #[tokio::test]
    async fn delete_transaction_on_unauthorised_user_returns_err() {}

    #[tokio::test]
    async fn delete_transaction_on_dberr_returns_err() {}
}
