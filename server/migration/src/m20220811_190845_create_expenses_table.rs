use entity::{currency_type, expense, predefined_expense, recurrence_type, user};

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
                    .table(expense::Entity)
                    .col(
                        ColumnDef::new(expense::Column::Id)
                            .big_unsigned()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(expense::Column::Name)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(expense::Column::Description)
                            .string_len(2000)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(expense::Column::Value)
                            .decimal_len(12, 2)
                            .not_null(),
                    )
                    .col(ColumnDef::new(expense::Column::StartDate).date().not_null())
                    .col(
                        ColumnDef::new(expense::Column::UserId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(expense::Column::CurrencyTypeId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(expense::Column::RecurrenceTypeId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(expense::Column::PredefinedExpenseId)
                            .big_unsigned()
                            .null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_expense-user")
                            .from_tbl(expense::Entity)
                            .from_col(expense::Column::UserId)
                            .to_tbl(user::Entity)
                            .to_col(user::Column::Id),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_expense-currency_type")
                            .from_tbl(expense::Entity)
                            .from_col(expense::Column::CurrencyTypeId)
                            .to_tbl(currency_type::Entity)
                            .to_col(currency_type::Column::Id),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_expense-recurrence_type")
                            .from_tbl(expense::Entity)
                            .from_col(expense::Column::RecurrenceTypeId)
                            .to_tbl(recurrence_type::Entity)
                            .to_col(recurrence_type::Column::Id),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_expense-predefined_expense")
                            .from_tbl(expense::Entity)
                            .from_col(expense::Column::PredefinedExpenseId)
                            .to_tbl(predefined_expense::Entity)
                            .to_col(predefined_expense::Column::Id),
                    )
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        expense::ActiveModel {
            id: Set(1),
            name: Set("Netflix for my little family".to_string()),
            description: Set("Cheapest monthly plan of Netflix - Maybe upgrade later".to_string()),
            value: Set(Decimal::new(2490, 2)),
            start_date: Set(NaiveDate::from_ymd_opt(2022, 9, 24).unwrap()),
            user_id: Set(1),
            currency_type_id: Set(1),
            recurrence_type_id: Set(1),
            predefined_expense_id: Set(Some(1)),
        }
        .insert(db)
        .await?;
        expense::ActiveModel {
            id: Set(2),
            name: Set("Synology C2 backup".to_string()),
            description: Set("Its not much but it keeps our photos safe".to_string()),
            value: Set(Decimal::new(3499, 2)),
            start_date: Set(NaiveDate::from_ymd_opt(2022, 3, 15).unwrap()),
            user_id: Set(1),
            currency_type_id: Set(2),
            recurrence_type_id: Set(2),
            predefined_expense_id: Set(None),
        }
        .insert(db)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(expense::Entity).to_owned())
            .await
    }
}
