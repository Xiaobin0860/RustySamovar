// Database Manager

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "trans_point")]
pub struct Model {
    #[sea_orm(primary_key, autoincrement = false)]
    pub uid: u32,
    pub scene_id: u32,
    pub point_id: u32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            _ => panic!("Unknown relation type!"),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
