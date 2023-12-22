pub use sea_orm_migration::prelude::*;

mod m20231222_021548_create_route_names;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231222_021548_create_route_names::Migration),
        ]
    }
}
