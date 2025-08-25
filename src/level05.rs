use std::time::Duration;

use bevy::app::Plugin;

use AppState::Running;
use LevelState::Level05;
use bevy::prelude::*;

use crate::{
    animation::{Animation, AnimationType},
    app_states::{AppState, LevelState},
    controls::PlayerControlled,
    health::{Health, Hearts},
    in_game::LevelFinished,
    sprites::{ExfilSprite, MySprite, SpawnSprite, SpriteSheetTile},
    tiles::TileCoordinate,
    tutorial::Tutorial,
};

// Constants
const NAME: &str = "level05";

// Plugin
pub struct Level05Plugin;

impl Plugin for Level05Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Level05), start_level05)
            .add_systems(
                Update,
                (
                    update_level05,
                    added_player_controlled,
                    added_tutorial_components,
                    check_for_exit_level05,
                )
                    .run_if(in_state(Running))
                    .run_if(in_state(Level05)),
            )
            .add_systems(OnExit(Level05), stop_level05);
    }
}

// Components

// Resources

// Events

// Systems
fn start_level05(mut spawn_sprite: EventWriter<SpawnSprite>) {
    debug!("starting {}", NAME);

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 0, y: 0, z: 0 },
        tile: SpriteSheetTile::Player01,
        color: Some(Color::linear_rgb(0.5, 0.5, 0.5)),
        ..default()
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 1, y: 1, z: 0 },
        tile: SpriteSheetTile::BrickWall01,
        ..default()
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 3, y: 1, z: 0 },
        tile: SpriteSheetTile::BrickWall01,
        ..default()
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 1, y: 2, z: 0 },
        tile: SpriteSheetTile::BrickWall01,
        ..default()
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 2, y: 2, z: 0 },
        tile: SpriteSheetTile::BrickWall01,
        ..default()
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 3, y: 2, z: 0 },
        tile: SpriteSheetTile::BrickWall01,
        ..default()
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 2, y: 1, z: 0 },
        tile: SpriteSheetTile::LevelExit01,
        color: Some(Color::linear_rgb(0.0, 0.5, 0.5)),
        ..default()
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 2, y: -1, z: 0 },
        tile: SpriteSheetTile::Heart,
        tutorial: true,
        ..default()
    });
}

fn added_player_controlled(
    mut commands: Commands,
    added_player_controlled: Query<Entity, Added<PlayerControlled>>,
) {
    for added in added_player_controlled.iter() {
        commands.entity(added).insert(Health {
            hearts: Hearts(1),
            max: Hearts(2),
        });
    }
}

fn added_tutorial_components(
    mut commands: Commands,
    added_tutorials: Query<Entity, Added<Tutorial>>,
) {
    for added in added_tutorials.iter() {
        commands.entity(added).insert((
            Animation::new(
                Timer::new(Duration::from_millis(400), TimerMode::Repeating),
                EaseFunction::SineInOut,
            ),
            AnimationType::Pulse,
        ));
    }
}

fn update_level05() {
    debug!("updating {}", NAME);
}

fn check_for_exit_level05(
    mut next_state: ResMut<NextState<AppState>>,
    players: Query<&TileCoordinate, (With<PlayerControlled>, Without<ExfilSprite>)>,
    exfils: Query<&TileCoordinate, (With<ExfilSprite>, Without<PlayerControlled>)>,
) {
    debug!("checking exit {}", NAME);
    for player_coordinate in players.iter() {
        for exfil_coordinate in exfils.iter() {
            if player_coordinate.eq2d(exfil_coordinate) {
                // TODO: smoother transition, maybe with animation on an event
                next_state.set(AppState::MainMenu);
            }
        }
    }
}

fn stop_level05(
    mut commands: Commands,
    sprites: Query<Entity, With<MySprite>>,
    mut finished: EventWriter<LevelFinished>,
) {
    debug!("stopping {}", NAME);
    for sprite in sprites.iter() {
        commands.entity(sprite).despawn();
    }
    finished.write(LevelFinished);
}

// helper functions
