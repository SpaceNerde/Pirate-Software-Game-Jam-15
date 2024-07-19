mod scene_handler;

use crate::scene_handler::SceneHandlerPlugin;

use bevy::prelude::*;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    #[default]
    PotionCrafting,
    Fighting,
    Map,
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.add_plugins((
            SceneHandlerPlugin,
        ));
    }
}
