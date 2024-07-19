use bevy::prelude::*;

pub struct SceneHandlerPlugin;

impl Plugin for SceneHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

#[derive(Component)]
struct PotionCraftingScene;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            ..default()
        },
        PotionCraftingScene,
    ));
}
