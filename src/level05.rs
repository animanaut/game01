use std::time::Duration;

use bevy::app::Plugin;

use AppState::Running;
use LevelState::Level05;
use bevy::prelude::*;

use crate::{
    animation::{Animation, AnimationType},
    app_states::{AppState, LevelState},
    controls::PlayerControlled,
    health::{Health, Hearts, PickedUpEmptyHeart, PickedUpHearts},
    in_game::{LevelFinished, LevelStarted},
    sprites::{ExfilSprite, MySprite, SpawnSprite, SpriteSheetTile},
    tiles::{DoorTile, TileCoordinate},
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
                    added_player_controlled,
                    added_tutorial_components,
                    check_for_exit_level05,
                )
                    .run_if(in_state(Running))
                    .run_if(in_state(Level05)),
            )
            .add_systems(
                Update,
                (picked_up_heart)
                    .run_if(on_event::<PickedUpHearts>)
                    .run_if(in_state(Level05)),
            )
            .add_systems(
                Update,
                (picked_up_empty_heart)
                    .run_if(on_event::<PickedUpEmptyHeart>)
                    .run_if(in_state(Level05)),
            )
            .add_systems(OnExit(Level05), stop_level05);
    }
}

// Components

// Resources

// Events

// Systems
fn start_level05(
    mut started: EventWriter<LevelStarted>,
    mut spawn_sprite: EventWriter<SpawnSprite>,
) {
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
        tile: SpriteSheetTile::MagicDoor,
        color: Some(Color::linear_rgb(0.0, 0.5, 0.5)),
        ..default()
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 2, y: -1, z: 0 },
        tile: SpriteSheetTile::Heart,
        ..default()
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 4, y: -1, z: 0 },
        tile: SpriteSheetTile::EmptyHeart,
        tutorial: true,
        ..default()
    });

    started.write(LevelStarted);
}

fn added_player_controlled(
    mut commands: Commands,
    added_player_controlled: Query<Entity, Added<PlayerControlled>>,
) {
    for added in added_player_controlled.iter() {
        commands.entity(added).insert(Health {
            hearts: Hearts(2),
            max: Hearts(2),
        });
    }
}

fn picked_up_heart(
    mut commands: Commands,
    mut pickups: EventReader<PickedUpHearts>,
    doors: Query<Entity, With<DoorTile>>,
    mut spawn_sprite: EventWriter<SpawnSprite>,
) {
    for _ in pickups.read() {
        if let Ok(door) = doors.single() {
            commands.entity(door).despawn();
            spawn_sprite.write(SpawnSprite {
                coordinate: TileCoordinate { x: 2, y: 1, z: 0 },
                tile: SpriteSheetTile::LevelExit01,
                ..default()
            });
        }
    }
}

fn picked_up_empty_heart(
    mut commands: Commands,
    mut pickups: EventReader<PickedUpEmptyHeart>,
    hearts: Query<Entity, With<Hearts>>,
) {
    for _ in pickups.read() {
        if let Ok(heart) = hearts.single() {
            commands.entity(heart).insert(Tutorial);
        }
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
