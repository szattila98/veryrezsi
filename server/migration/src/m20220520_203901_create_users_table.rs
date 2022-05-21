use entity::user;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220520_203901_create_users_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(user::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(user::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(user::Column::Email)
                            .string_len(320)
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(user::Column::Username).string().not_null())
                    .col(ColumnDef::new(user::Column::PwHash).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(user::Entity).to_owned())
            .await
    }
}
