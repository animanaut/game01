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

mod app_states;
mod controls;
mod in_game;
mod keyboard_controller;
mod level01;
mod level02;
mod main_menu;
mod splash;
mod sprites;
mod tiles;

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
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
