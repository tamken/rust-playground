//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;

// #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, serde::Serialize, serde::Deserialize)]
#[sea_orm(table_name = "dept")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)] // NOTE: using json
    pub deptno: i32,
    pub dname: String,
    pub loc: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::emp::Entity")]
    Emp,
}

impl Related<super::emp::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Emp.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}