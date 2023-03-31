//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "category")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub category_id: i16,
    pub name: String,
    pub last_update: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::film::Entity> for Entity {
    fn to() -> RelationDef {
        super::film_category::Relation::Film.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::film_category::Relation::Category.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
