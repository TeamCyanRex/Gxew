#[doc = "gxew"]
pub mod core;
#[path = "./multi-media/mod.rs"]
pub mod multi_media;
pub mod physics;
pub mod prelude;
pub mod spirits;
pub mod transform;
use crate::core::game_center::GameCenter;

#[macro_use(lazy_static)]
extern crate lazy_static;
pub fn init() -> GameCenter {
    GameCenter::init()
}
