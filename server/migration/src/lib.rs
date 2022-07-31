pub use sea_orm_migration::prelude::*;

mod m20220520_203901_create_users_table;
mod m20220707_222235_create_account_activation_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220520_203901_create_users_table::Migration),
            Box::new(m20220707_222235_create_account_activation_table::Migration),
        ]
    }
}
