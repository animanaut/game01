use app_states::{AppState, AppStatesPlugin};
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use controls::ControlsPlugin;
use in_game::InGamePlugin;
use keyboard_controller::KeyboardControllerPlugin;
use level01::Level01Plugin;
use level02::Level02Plugin;
use main_menu::MainMenuPlugin;
use splash::SplashPlugin;
use sprites::SpritesPlugin;
use tiles::TilesPlugin;

use crate::animation::AnimationPlugin;
use crate::game_camera::GameCameraPlugin;
use crate::gold::GoldPlugin;
use crate::level03::Level03Plugin;
use crate::movement::MovementPlugin;
use crate::tutorial::TutorialPlugin;

mod animation;
mod app_states;
mod controls;
mod game_camera;
mod gold;
mod in_game;
mod keyboard_controller;
mod level01;
mod level02;
mod level03;
mod main_menu;
mod movement;
mod splash;
mod sprites;
mod tiles;
mod tutorial;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics in web builds on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        )
        .add_plugins((
            AnimationPlugin,
            AppStatesPlugin,
            MainMenuPlugin,
            SplashPlugin,
            InGamePlugin,
            SpritesPlugin,
            ControlsPlugin,
            KeyboardControllerPlugin,
            TilesPlugin,
            Level01Plugin,
            Level02Plugin,
            Level03Plugin,
            GameCameraPlugin,
            GoldPlugin,
            TutorialPlugin,
        ))
        .add_plugins((MovementPlugin,))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
