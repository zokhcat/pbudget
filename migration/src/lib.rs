pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table_user;
mod m20220101_000002_create_table_budget;
mod m20220101_000003_create_table_expense;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table_user::Migration),
            Box::new(m20220101_000002_create_table_budget::Migration),
            Box::new(m20220101_000003_create_table_expense::Migration),
        ]
    }
}
