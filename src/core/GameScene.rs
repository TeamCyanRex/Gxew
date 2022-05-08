use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::spirits::game_obj::GameObj;
pub struct Gamescene{
    pub scene_name: String,
    pub game_objs: HashMap<String,Rc<RefCell<Option<Box<dyn GameObj>>>>>
}