use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::any::Any;
#[allow(unused_imports)]
use std::collections::LinkedList;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    //#[sea_orm(ignore)]
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "crate::entities::post::Entity")]
    Post,
    #[sea_orm(has_many = "crate::entities::book::Entity")]
    Book,
}

impl Related<crate::entities::post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl Related<crate::entities::book::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Book.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
