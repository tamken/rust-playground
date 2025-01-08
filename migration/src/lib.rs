pub use sea_orm_migration::prelude::*;

mod m20241223_085007_dept_table;
mod m20241223_085012_emp_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241223_085007_dept_table::Migration),
            Box::new(m20241223_085012_emp_table::Migration),
        ]
    }
}
