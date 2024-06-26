use sea_orm_migration::prelude::*;

mod m20241204_000001_create_bakery_table;
mod m20241204_000002_create_chef_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241204_000001_create_bakery_table::Migration),
            Box::new(m20241204_000002_create_chef_table::Migration),
        ]
    }
}
