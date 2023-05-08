use entity::{currency, predefined_expense, recurrence};

use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::entity::ActiveModelTrait;
use sea_orm_migration::sea_orm::prelude::Decimal;
use sea_orm_migration::sea_orm::Set;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(predefined_expense::Entity)
                    .col(
                        ColumnDef::new(predefined_expense::Column::Id)
                            .big_unsigned()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(predefined_expense::Column::Name)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(predefined_expense::Column::Description)
                            .string_len(2000)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(predefined_expense::Column::Value)
                            .decimal_len(12, 2)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(predefined_expense::Column::CurrencyId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(predefined_expense::Column::RecurrenceId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_predefined_expense-currency")
                            .from_tbl(predefined_expense::Entity)
                            .from_col(predefined_expense::Column::CurrencyId)
                            .to_tbl(currency::Entity)
                            .to_col(currency::Column::Id),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_predefined_expense-recurrence")
                            .from_tbl(predefined_expense::Entity)
                            .from_col(predefined_expense::Column::RecurrenceId)
                            .to_tbl(recurrence::Entity)
                            .to_col(recurrence::Column::Id),
                    )
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        predefined_expense::ActiveModel {
            id: Set(1),
            name: Set("Netflix Basic".to_string()),
            description: Set("Cheapest monthly plan of Netflix".to_string()),
            value: Set(Decimal::new(249, 1)),
            currency_id: Set(1),
            recurrence_id: Set(1),
        }
        .insert(db)
        .await?;
        predefined_expense::ActiveModel {
            id: Set(2),
            name: Set("Netflix Standard".to_string()),
            description: Set("Budget monthly plan of Netflix".to_string()),
            value: Set(Decimal::new(349, 1)),
            currency_id: Set(1),
            recurrence_id: Set(1),
        }
        .insert(db)
        .await?;
        predefined_expense::ActiveModel {
            id: Set(3),
            name: Set("Netflix Premium".to_string()),
            description: Set("Fully flashed monthly plan of Netflix".to_string()),
            value: Set(Decimal::new(449, 1)),
            currency_id: Set(1),
            recurrence_id: Set(1),
        }
        .insert(db)
        .await?;
        predefined_expense::ActiveModel {
            id: Set(4),
            name: Set("IntelliJ IDEA Ultimate".to_string()),
            description: Set(
                "The most popular IDE of JetBrains with all of its features".to_string()
            ),
            value: Set(Decimal::new(499, 2)),
            currency_id: Set(2),
            recurrence_id: Set(2),
        }
        .insert(db)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(predefined_expense::Entity).to_owned())
            .await
    }
}
