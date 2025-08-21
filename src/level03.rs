use std::time::Duration;

use bevy::app::Plugin;

use AppState::{MainMenu, Running};
use LevelState::Level03;
use bevy::prelude::*;

use crate::{
    animation::{Animation, AnimationType},
    app_states::{AppState, LevelState},
    controls::{Down, Left, PlayerControlled, Right, Up},
    in_game::LevelFinished,
    sprites::{ExfilSprite, MySprite, SpawnSprite, SpriteSheetTile},
    tiles::TileCoordinate,
    tutorial::{Tutorial, TutorialCountdown},
};

// Constants
const NAME: &str = "level03";

// Plugin
pub struct Level03Plugin;

impl Plugin for Level03Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Level03), start_level03)
            .add_systems(
                Update,
                (
                    update_level03,
                    added_tutorial_components,
                    countdown_tutorial,
                    check_for_exit_level03,
                )
                    .run_if(in_state(Running))
                    .run_if(in_state(Level03)),
            )
            .add_systems(OnExit(Level03), stop_level03);
    }
}

// Components

// Resources

// Events

// Systems
fn start_level03(mut spawn_sprite: EventWriter<SpawnSprite>) {
    debug!("starting {}", NAME);

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 0, y: 0, z: 0 },
        tile: SpriteSheetTile::Player01,
        color: Some(Color::linear_rgb(0.5, 0.5, 0.5)),
        ..default()
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 1, y: 2, z: 3 },
        tile: SpriteSheetTile::LevelExit01,
        color: Some(Color::linear_rgb(0.0, 0.5, 0.5)),
        ..default()
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 1, y: 1, z: 0 },
        tile: SpriteSheetTile::BrickWall01,
        tutorial: true,
        ..default()
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 0, y: 1, z: 0 },
        tile: SpriteSheetTile::BrickWall01,
        tutorial: true,
        ..default()
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 2, y: 1, z: 0 },
        tile: SpriteSheetTile::BrickWall01,
        tutorial: true,
        ..default()
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 0, y: 2, z: 0 },
        tile: SpriteSheetTile::BrickWall01,
        tutorial: true,
        ..default()
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 2, y: 2, z: 0 },
        tile: SpriteSheetTile::BrickWall01,
        tutorial: true,
        ..default()
    });
}

fn update_level03() {
    debug!("updating {}", NAME);
}

fn added_tutorial_components(
    mut commands: Commands,
    added_tutorials: Query<Entity, Added<Tutorial>>,
) {
    for added in added_tutorials.iter() {
        commands.entity(added).insert(TutorialCountdown::new(0));
        commands.entity(added).insert((
            Animation::new(
                Timer::new(Duration::from_millis(400), TimerMode::Once),
                EaseFunction::SineInOut,
            ),
            AnimationType::Pulse,
        ));
    }
}

fn countdown_tutorial(
    mut commands: Commands,
    mut right: EventReader<Right>,
    mut left: EventReader<Left>,
    mut up: EventReader<Up>,
    mut down: EventReader<Down>,
    countdowns: Query<Entity, With<Tutorial>>,
) {
    let mut countdown = false;
    for _ in right.read() {
        countdown = true;
    }

    for _ in left.read() {
        countdown = true;
    }

    for _ in up.read() {
        countdown = true;
    }

    for _ in down.read() {
        countdown = true;
    }

    if countdown {
        for c in countdowns.iter() {
            // pulse on every step
            commands.entity(c).insert((
                Animation::new(
                    Timer::new(Duration::from_millis(100), TimerMode::Once),
                    EaseFunction::SineInOut,
                ),
                AnimationType::Pulse,
            ));
        }
    }
}

fn check_for_exit_level03(
    mut next_state: ResMut<NextState<AppState>>,
    players: Query<&TileCoordinate, (With<PlayerControlled>, Without<ExfilSprite>)>,
    exfils: Query<&TileCoordinate, (With<ExfilSprite>, Without<PlayerControlled>)>,
) {
    debug!("checking exit {}", NAME);
    for player_coordinate in players.iter() {
        for exfil_coordinate in exfils.iter() {
            if player_coordinate.eq2d(exfil_coordinate) {
                // TODO: smoother transition, maybe with animation on an event
                next_state.set(MainMenu);
            }
        }
    }
}

fn stop_level03(
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

// tests
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    //use super::*;
    //use std::borrow::BorrowMut;

    /*
    #[test]
    fn should_test_something() {
        // given
        //let mut app = App::new();

        // when
        //app.add_event::<HealthDamageReceived>();
        //app.add_systems(Update, damage_received_listener);
        //let entity = app.borrow_mut().world().spawn(Health(100)).id();
        //app.borrow_mut().world().resource_mut::<Events<HealthDamageReceived>>().send(HealthDamageReceived { entity, damage: 10 });
        //app.update();

        // then
        //assert!(app.world().get::<Health>(entity).is_some());
        //assert_eq!(app.world().get::<Health>(entity).unwrap().0, 90);
    }
    */
}
