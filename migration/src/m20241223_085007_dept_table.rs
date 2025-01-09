use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .create_table(
                Table::create()
                    .table(Dept::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Dept::Deptno)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Dept::Dname).string_len(14).not_null())
                    .col(ColumnDef::new(Dept::Loc).string_len(13).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .drop_table(Table::drop().table(Dept::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Dept {
    Table,
    Deptno,
    Dname,
    Loc,
}
