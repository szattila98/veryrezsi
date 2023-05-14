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
mod tests {
    use assert2::check;
    use entity::Id;
    use migration::DbErr;
    use sea_orm::entity::prelude::*;
    use sea_orm::{DatabaseBackend, DeriveActiveModelBehavior, DeriveEntityModel, MockDatabase};

    use crate::logic::find_entity_by_id;

    const TEST_STR: &str = "test";
    const TEST_ID: u64 = 1;

    const fn TEST_DB_ERROR() -> DbErr {
        DbErr::Custom(TEST_STR.to_string())
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
