use entity::{currency, expense, transaction};

use chrono::NaiveDate;
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
                    .table(transaction::Entity)
                    .col(
                        ColumnDef::new(transaction::Column::Id)
                            .big_unsigned()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(transaction::Column::DonorName)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(transaction::Column::Value)
                            .decimal_len(12, 2)
                            .not_null(),
                    )
                    .col(ColumnDef::new(transaction::Column::Date).date().not_null())
                    .col(
                        ColumnDef::new(transaction::Column::CurrencyId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(transaction::Column::ExpenseId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_transaction-currency")
                            .from_tbl(transaction::Entity)
                            .from_col(transaction::Column::CurrencyId)
                            .to_tbl(currency::Entity)
                            .to_col(currency::Column::Id),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_transaction-expense")
                            .from_tbl(transaction::Entity)
                            .from_col(transaction::Column::ExpenseId)
                            .to_tbl(expense::Entity)
                            .to_col(expense::Column::Id),
                    )
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        transaction::ActiveModel {
            id: Set(1),
            donor_name: Set("Kate".to_string()),
            value: Set(Decimal::new(5, 2)),
            date: Set(NaiveDate::from_ymd_opt(2022, 9, 29).unwrap()),
            currency_id: Set(2),
            expense_id: Set(1),
        }
        .insert(db)
        .await?;
        transaction::ActiveModel {
            id: Set(2),
            donor_name: Set("David".to_string()),
            value: Set(Decimal::new(7, 0)),
            date: Set(NaiveDate::from_ymd_opt(2022, 10, 23).unwrap()),
            currency_id: Set(2),
            expense_id: Set(1),
        }
        .insert(db)
        .await?;
        transaction::ActiveModel {
            id: Set(3),
            donor_name: Set("Wifey".to_string()),
            value: Set(Decimal::new(50, 0)),
            date: Set(NaiveDate::from_ymd_opt(2022, 4, 12).unwrap()),
            currency_id: Set(2),
            expense_id: Set(2),
        }
        .insert(db)
        .await?;
        transaction::ActiveModel {
            id: Set(4),
            donor_name: Set("My colleague who use exotic Hungarian Forint".to_string()),
            value: Set(Decimal::new(1000, 0)),
            date: Set(NaiveDate::from_ymd_opt(2022, 5, 13).unwrap()),
            currency_id: Set(1),
            expense_id: Set(2),
        }
        .insert(db)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(transaction::Entity).to_owned())
            .await
    }
}
