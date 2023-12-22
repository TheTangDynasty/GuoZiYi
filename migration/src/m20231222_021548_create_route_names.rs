use sea_orm_migration::prelude::*;

use ::entity::route_names;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let schema = manager.get_database_backend();

        manager
            .create_table(
                Table::create()
                    .table(route_names::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(route_names::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(route_names::Column::Name)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(route_names::Column::CreatedAt).integer())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(route_names::Entity).to_owned())
            .await
    }
}
