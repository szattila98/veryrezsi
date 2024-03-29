use entity::user;

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
                    .table(user::Entity)
                    .col(
                        ColumnDef::new(user::Column::Id)
                            .big_unsigned()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(
                        ColumnDef::new(user::Column::Email)
                            .string_len(320)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(user::Column::Username)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(user::Column::PwHash)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(user::Column::Activated)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await?;

        // Dummy user
        let db = manager.get_connection();
        user::ActiveModel {
            id: Set(1),
            email: Set("bob@ross.com".to_string()),
            username: Set("happylittleclouds".to_string()),
            pw_hash: Set(
                "$2b$10$YvSfR107VspgYn9AoveuTOQ.GRjj0UvRI1w9YlgA7oMz9uPLBNGVS".to_string(),
            ),
            activated: Set(true),
        }
        .insert(db)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(user::Entity).to_owned())
            .await
    }
}
