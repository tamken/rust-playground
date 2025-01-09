use crate::m20241223_085007_dept_table::Dept;
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
                    .table(Emp::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Emp::Empno)
                            .integer()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Emp::Ename).string_len(10).not_null())
                    .col(ColumnDef::new(Emp::Job).string_len(9).not_null())
                    .col(ColumnDef::new(Emp::Mgr).integer())
                    .col(ColumnDef::new(Emp::Hiredate).date().not_null())
                    .col(ColumnDef::new(Emp::Sal).decimal_len(7, 2).not_null())
                    .col(ColumnDef::new(Emp::Comm).decimal_len(7, 2))
                    .col(ColumnDef::new(Emp::Deptno).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_deptno")
                            .from(Emp::Table, Emp::Deptno)
                            .to(Dept::Table, Dept::Deptno)
                            // .on_delete(ForeignKeyAction::Cascade)
                            // .on_update(ForeignKeyAction::Cascade),
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // todo!();

        manager
            .drop_table(Table::drop().table(Emp::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Emp {
    Table,
    Empno,
    Ename,
    Job,
    Mgr,
    Hiredate,
    Sal,
    Comm,
    Deptno,
}
