use entity::recurrence_type;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::entity::ActiveModelTrait;
use sea_orm_migration::sea_orm::Set;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(recurrence_type::Entity)
                    .col(
                        ColumnDef::new(recurrence_type::Column::Id)
                            .big_integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(recurrence_type::Column::Name)
                            .string_len(255)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(recurrence_type::Column::PerYear)
                            .double()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        recurrence_type::ActiveModel {
            id: Set(1),
            name: Set("Monthly".to_string()),
            per_year: Set(12.0),
        }
        .insert(db)
        .await?;
        recurrence_type::ActiveModel {
            id: Set(2),
            name: Set("Annual".to_string()),
            per_year: Set(1.0),
        }
        .insert(db)
        .await?;
        recurrence_type::ActiveModel {
            id: Set(3),
            name: Set("Two yearly".to_string()),
            per_year: Set(0.5),
        }
        .insert(db)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(recurrence_type::Entity).to_owned())
            .await
    }
}
