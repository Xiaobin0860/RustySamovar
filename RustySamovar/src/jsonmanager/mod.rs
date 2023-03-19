mod json_manager;

pub use self::json_manager::JsonManager;

mod avatar;
mod avatar_skill;
mod avatar_skill_depot;
mod entity_curve;
mod gadget_prop;
mod gather;
mod material;
mod monster;
mod proud_skill;
mod reliquary;
mod scene;
mod shop_goods;
mod shop_rotate;
mod teleport_point;
mod weapon;
mod world_level;

pub use entity_curve::{CurveInfo, EntityCurve};
pub use shop_goods::ShopGoods;
