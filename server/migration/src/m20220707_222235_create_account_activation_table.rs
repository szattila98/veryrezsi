use entity::{account_activation, user};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(account_activation::Entity)
                    .col(
                        ColumnDef::new(account_activation::Column::Id)
                            .big_integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(account_activation::Column::Token)
                            .string_len(36)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(account_activation::Column::UserId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(account_activation::Column::Expiration)
                            .timestamp()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_activation-user")
                            .from_tbl(account_activation::Entity)
                            .from_col(account_activation::Column::UserId)
                            .to_tbl(user::Entity)
                            .to_col(user::Column::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(account_activation::Entity).to_owned())
            .await
    }
}
