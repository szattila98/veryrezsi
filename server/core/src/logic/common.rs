use migration::DbErr;
use sea_orm::{DatabaseConnection, EntityTrait, PrimaryKeyTrait};

pub static DATE_FORMAT: &str = "%d-%m-%Y";

pub async fn find_entity_by_id<E: EntityTrait>(
    conn: &DatabaseConnection,
    id: <<E as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType,
) -> Result<Option<E::Model>, DbErr> {
    E::find_by_id(id).one(conn).await
}

#[cfg(test)]
pub mod tests {
    use assert2::check;
    use chrono::NaiveDate;
    use entity::{Id, currency, recurrence, expense, predefined_expense, transaction};
    use migration::DbErr;
    use sea_orm::entity::prelude::*;
    use sea_orm::{DatabaseBackend, DeriveActiveModelBehavior, DeriveEntityModel, MockDatabase};

    use crate::logic::find_entity_by_id;

    pub const TEST_STR: &str = "test";
    pub const TEST_ID: u64 = 1;
    pub const TEST_FLOAT: f64 = 1.0;
    pub const TEST_DATE: &str = "06-08-1998";

    pub fn TEST_DB_ERROR() -> DbErr {
        DbErr::Custom(TEST_STR.to_string())
    }

    pub fn TEST_CURRENCY() -> currency::Model {
        return currency::Model {
            id: TEST_ID,
            abbreviation: TEST_STR.to_string(),
            name: TEST_STR.to_string(),
        }
    }

    pub fn TEST_RECURRENCE() -> recurrence::Model {
        return recurrence::Model {
            id: TEST_ID,
            name: TEST_STR.to_string(),
            per_year: TEST_FLOAT,
        }
    }

    pub fn TEST_EXPENSE() -> expense::Model {
        return expense::Model {
            id: TEST_ID,
            name: TEST_STR.to_string(),
            description: TEST_STR.to_string(),
            value: TEST_DECIMAL(),
            start_date: NaiveDate::MIN,
            user_id: TEST_ID,
            currency_id: TEST_ID,
            recurrence_id: TEST_ID,
            predefined_expense_id: Some(TEST_ID),
        }   
    }

    pub fn TEST_PREDEFINED_EXPENSE() -> predefined_expense::Model {
        return predefined_expense::Model {
            id: TEST_ID,
            name: TEST_STR.to_string(),
            description: TEST_STR.to_string(),
            value: TEST_DECIMAL(),
            currency_id: TEST_ID,
            recurrence_id: TEST_ID,
        }
    }

    pub fn TEST_TRANSACTION() -> transaction::Model {
        return transaction::Model {
            id: TEST_ID,
            donor_name: TEST_STR.to_string(),
            value: TEST_DECIMAL(),
            date: NaiveDate::MIN,
            currency_id: TEST_ID,
            expense_id: TEST_ID,
        }
    }

    pub fn TEST_TRANSACTION_2() -> transaction::Model {
        return transaction::Model {
            id: TEST_TRANSACTION().id + 1,
            ..TEST_TRANSACTION()
        }
    }

    pub fn TEST_DECIMAL() -> Decimal {
        Decimal::new(1, 2)
    }

    #[derive(Clone, Debug, PartialEq, Eq, DeriveActiveModelBehavior, DeriveEntityModel)]
    #[sea_orm(table_name = "test")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: Id,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    #[tokio::test]
    async fn find_entity_by_id_all_cases() {
        let mock_model = Model { id: TEST_ID };
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![vec![mock_model.clone()], vec![]])
            .append_query_errors(vec![TEST_DB_ERROR()])
            .into_connection();

        let (model, not_found, db_error) = tokio::join!(
            find_entity_by_id::<Entity>(&conn, TEST_ID),
            find_entity_by_id::<Entity>(&conn, TEST_ID),
            find_entity_by_id::<Entity>(&conn, TEST_ID)
        );

        check!(model == Ok(Some(mock_model)));
        check!(not_found == Ok(None));
        check!(db_error == Err(TEST_DB_ERROR()));
    }
}
