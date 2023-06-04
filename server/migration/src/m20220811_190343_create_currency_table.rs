use entity::currency;

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
                    .table(currency::Entity)
                    .col(
                        ColumnDef::new(currency::Column::Id)
                            .big_unsigned()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(currency::Column::Abbreviation)
                            .string_len(255)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(currency::Column::Name)
                            .string_len(255)
                            .not_null()
                            .unique_key(),
                    )
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        // Chad forint
        currency::ActiveModel {
            id: Set(1),
            abbreviation: Set("HUF".to_string()),
            name: Set("base.currencies.huf".to_string()),
        }
        .insert(db)
        .await?;
        // Virgin euro
        currency::ActiveModel {
            id: Set(2),
            abbreviation: Set("EUR".to_string()),
            name: Set("base.currencies.eur".to_string()),
        }
        .insert(db)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(currency::Entity).to_owned())
            .await
    }
}
