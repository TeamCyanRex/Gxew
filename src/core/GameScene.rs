use crate::core::toolkits::{Store, Tag};
use crate::spirits::game_obj::GameObj;
use std::collections::HashMap;
use yew::prelude::*;
pub type Id = u64;
pub enum GxewSceneMessage {}
#[derive(Debug, PartialEq, Properties)]
pub struct GxewSceneProps {
    pub scene_name: &'static str,
}
impl GxewSceneProps {
    fn new(name: &'static str) -> GxewSceneProps {
        GxewSceneProps { scene_name: name }
    }
}
pub struct Gamescene {
    pub props: GxewSceneProps,
    pub gameobj_name_id_table: HashMap<String, Id>,
    pub gameobjs: HashMap<Id, Store<dyn GameObj>>,
    pub gameobj_properties_table: HashMap<Id, HashMap<Tag, bool>>,
}
impl Gamescene {
    fn new(name: &'static str) -> Self {
        Self {
            props: GxewSceneProps::new(name),
            gameobj_name_id_table: HashMap::new(),
            gameobjs: HashMap::new(),
            gameobj_properties_table: HashMap::new(),
        }
    }
}
impl Component for Gamescene {
    type Message = GxewSceneMessage;
    type Properties = GxewSceneProps;
    fn create(ctx: &Context<Self>) -> Self {
        Gamescene::new(ctx.props().scene_name)
    }
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}
    fn view(&self, ctx: &Context<Self>) -> Html {
        html!()
    }
    fn destroy(&mut self, ctx: &Context<Self>) {}
}
