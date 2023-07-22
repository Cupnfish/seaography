//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "inventory")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub inventory_id: i32,
    pub film_id: i16,
    pub store_id: i16,
    pub last_update: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::film::Entity",
        from = "Column::FilmId",
        to = "super::film::Column::FilmId",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    Film,
    #[sea_orm(has_many = "super::rental::Entity")]
    Rental,
    #[sea_orm(
        belongs_to = "super::store::Entity",
        from = "Column::StoreId",
        to = "super::store::Column::StoreId",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    Store,
}

impl Related<super::film::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Film.def()
    }
}

impl Related<super::rental::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Rental.def()
    }
}

impl Related<super::store::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Store.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    #[sea_orm(entity = "super::film::Entity")]
    Film,
    #[sea_orm(entity = "super::rental::Entity")]
    Rental,
    #[sea_orm(entity = "super::store::Entity")]
    Store,
}
