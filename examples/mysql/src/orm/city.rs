//! SeaORM Entity. Generated by sea-orm-codegen 0.9.1

use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "city"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel)]
pub struct Model {
    pub city_id: u16,
    pub city: String,
    pub country_id: u16,
    pub last_update: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    CityId,
    City,
    CountryId,
    LastUpdate,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    CityId,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = u16;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Country,
    Address,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::CityId => ColumnType::SmallUnsigned.def(),
            Self::City => ColumnType::String(Some(50u32)).def(),
            Self::CountryId => ColumnType::SmallUnsigned.def(),
            Self::LastUpdate => ColumnType::Timestamp.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Country => Entity::belongs_to(super::country::Entity)
                .from(Column::CountryId)
                .to(super::country::Column::CountryId)
                .into(),
            Self::Address => Entity::has_many(super::address::Entity).into(),
        }
    }
}

impl Related<super::country::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Country.def()
    }
}

impl Related<super::address::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Address.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
