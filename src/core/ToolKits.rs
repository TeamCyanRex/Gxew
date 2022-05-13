use lazy_static::lazy_static;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
pub type Store<T> = Rc<RefCell<Option<Box<T>>>>;
pub type Tag = u64;
pub fn to_store<T>(src: T) -> Store<T> {
    Rc::new(RefCell::new(Some(Box::new(src))))
}
fn register_properties() -> HashMap<&'static str, Tag> {
    let mut result = HashMap::new();
    result.insert("Transform", 0);
    result.insert("Input", 1);
    result.insert("Image", 2);
    result.insert("Anime", 3);
    result.insert("Rigid", 4);
    result.insert("Friction", 5);
    result.insert("Audio", 6);
    result
}
lazy_static! {
    pub static ref GAME_PROPERTIES_NAME_TAG_TABLE: HashMap<&'static str, Tag> =
        register_properties();
}
