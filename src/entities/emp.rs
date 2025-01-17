//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;

// #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "emp")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)] // NOTE: using json
    pub empno: i32,
    pub ename: String,
    pub job: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mgr: Option<i32>,
    pub hiredate: Date,
    #[sea_orm(column_type = "Decimal(Some((7, 2)))")]
    pub sal: Decimal,
    #[sea_orm(column_type = "Decimal(Some((7, 2)))", nullable)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comm: Option<Decimal>,
    pub deptno: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::dept::Entity",
        from = "Column::Deptno",
        to = "super::dept::Column::Deptno",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    Dept,
}

impl Related<super::dept::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Dept.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
