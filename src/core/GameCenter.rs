use crate::core::game_scene::{self, Gamescene};
use crate::core::toolkits::{to_store, Store};
use yew::prelude::*;
pub enum GxewCenterMessage {}
#[derive(Debug, Properties, PartialEq, Default)]
pub struct GxewCenterProps {}
pub struct GameCenter {
    pub game_scenes: Vec<Store<Gamescene>>,
    pub active_scenes:Option<Store<Gamescene>>
}
impl GameCenter {
    pub fn init() -> Self {
        GameCenter {
            game_scenes: vec![],
            active_scenes: None
        }
    }
    pub fn pop_scene(&mut self) -> Option<Store<Gamescene>> {
        self.game_scenes.pop()
    }
    fn push_scene(&mut self, scene: Store<Gamescene>) {
        self.game_scenes.push(scene);
    }
    pub fn service(&mut self, game_sence: Gamescene) -> &mut Self {
        self.push_scene(to_store(game_sence));
        self
    }
    pub fn run(&self) {
        yew::start_app::<Self>();
    }
}
impl Component for GameCenter {
    type Message = GxewCenterMessage;
    type Properties = GxewCenterProps;
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        true
    }
    fn create(ctx: &Context<Self>) -> Self {
        GameCenter::init()
    }
    fn destroy(&mut self, ctx: &Context<Self>) {}
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {}
    fn view(&self, ctx: &Context<Self>) -> Html {
        html!()
    }
}
