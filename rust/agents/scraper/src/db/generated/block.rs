//! SeaORM Entity. Generated by sea-orm-codegen 0.9.3

use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "block"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel)]
pub struct Model {
    pub id: i64,
    pub time_created: TimeDateTime,
    pub domain: i32,
    pub hash: String,
    pub height: i64,
    pub timestamp: TimeDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    Id,
    TimeCreated,
    Domain,
    Hash,
    Height,
    Timestamp,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    Id,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i64;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Domain,
    Transaction,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::Id => ColumnType::BigInteger.def(),
            Self::TimeCreated => ColumnType::DateTime.def(),
            Self::Domain => ColumnType::Integer.def(),
            Self::Hash => ColumnType::String(Some(64u32)).def().unique(),
            Self::Height => ColumnType::BigInteger.def(),
            Self::Timestamp => ColumnType::DateTime.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Domain => Entity::belongs_to(super::domain::Entity)
                .from(Column::Domain)
                .to(super::domain::Column::Id)
                .into(),
            Self::Transaction => Entity::has_many(super::transaction::Entity).into(),
        }
    }
}

impl Related<super::domain::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Domain.def()
    }
}

impl Related<super::transaction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Transaction.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
