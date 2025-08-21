use std::time::Duration;

use bevy::app::Plugin;

use AppState::Running;
use LevelState::Level01;
use LevelState::Level02;
use bevy::prelude::*;

use crate::animation::Animation;
use crate::animation::AnimationType;
use crate::controls::{Down, Left, Right, Up};
use crate::in_game::LevelFinished;
use crate::tutorial::CountDownFinished;
use crate::tutorial::CountDownTutorialCounter;
use crate::tutorial::Tutorial;
use crate::tutorial::TutorialCountdown;
use crate::{
    app_states::{AppState, LevelState},
    controls::PlayerControlled,
    sprites::{ExfilSprite, MySprite, SpawnSprite, SpriteSheetTile},
    tiles::TileCoordinate,
};

// Constants
const NAME: &str = "level01";

// Plugin
pub struct Level01Plugin;

impl Plugin for Level01Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Level01), start_level01)
            .add_systems(
                Update,
                (
                    update_level01,
                    added_tutorial_components,
                    countdown_tutorial,
                    countdown_tutorial_finished,
                    check_for_exit_level01,
                )
                    .run_if(in_state(Running))
                    .run_if(in_state(Level01)),
            )
            .add_systems(OnExit(Level01), stop_level01);
    }
}

// Components

// Resources

// Events

// Systems
fn start_level01(mut spawn_sprite: EventWriter<SpawnSprite>) {
    debug!("starting {}", NAME);

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 0, y: 0, z: 0 },
        tile: SpriteSheetTile::Player01,
        ..default()
    });

    /*
        spawn_sprite.write(SpawnSprite {
            coordinate: TileCoordinate { x: -4, y: 0, z: 1 },
            tile: Tile::LeftDigiPadRound,
            tutorial: true,
            ..default()
        });

        spawn_sprite.write(SpawnSprite {
            coordinate: TileCoordinate { x: 4, y: 0, z: 1 },
            tile: Tile::RightDigiPadRound,
            tutorial: true,
            ..default()
        });

        spawn_sprite.write(SpawnSprite {
            coordinate: TileCoordinate { x: -2, y: 2, z: 1 },
            tile: Tile::UpDigiPadRound,
            tutorial: true,
            ..default()
        });

        spawn_sprite.write(SpawnSprite {
            coordinate: TileCoordinate { x: -2, y: -2, z: 1 },
            tile: Tile::DownDigiPadRound,
            tutorial: true,
            ..default()
        });
    */
    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: -3, y: 0, z: 1 },
        tile: SpriteSheetTile::A,
        tutorial: true,
        ..default()
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 3, y: 0, z: 1 },
        tile: SpriteSheetTile::D,
        tutorial: true,
        ..default()
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: -2, y: 1, z: 1 },
        tile: SpriteSheetTile::W,
        tutorial: true,
        ..default()
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: -2, y: -1, z: 1 },
        tile: SpriteSheetTile::S,
        tutorial: true,
        ..default()
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 2, y: 0, z: -1 },
        tile: SpriteSheetTile::LevelExit01,
        ..default()
    });
}

fn added_tutorial_components(
    mut commands: Commands,
    added_tutorials: Query<Entity, Added<Tutorial>>,
) {
    for added in added_tutorials.iter() {
        commands.entity(added).insert(TutorialCountdown::new(4));
        commands.entity(added).insert((
            Animation::new(
                Timer::new(Duration::from_millis(400), TimerMode::Repeating),
                EaseFunction::SineInOut,
            ),
            AnimationType::Pulse,
        ));
    }
}

fn countdown_tutorial(
    mut right: EventReader<Right>,
    mut left: EventReader<Left>,
    mut up: EventReader<Up>,
    mut down: EventReader<Down>,
    countdowns: Query<Entity, With<TutorialCountdown>>,
    mut countdown_event: EventWriter<CountDownTutorialCounter>,
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
            countdown_event.write(CountDownTutorialCounter(c));
        }
    }
}

fn countdown_tutorial_finished(
    mut commands: Commands,
    mut finished: EventReader<CountDownFinished>,
) {
    for f in finished.read() {
        commands.entity(f.0).despawn();
    }
}

fn update_level01() {
    debug!("updating {}", NAME);
}

fn check_for_exit_level01(
    mut next_state: ResMut<NextState<LevelState>>,
    players: Query<&TileCoordinate, (With<PlayerControlled>, Without<ExfilSprite>)>,
    exfils: Query<&TileCoordinate, (With<ExfilSprite>, Without<PlayerControlled>)>,
) {
    debug!("checking exit {}", NAME);
    for player_coordinate in players.iter() {
        for exfil_coordinate in exfils.iter() {
            if player_coordinate.eq2d(exfil_coordinate) {
                // TODO: smoother transition, maybe with animation on an event
                debug!("changing LevelState to {:?}", Level02);
                next_state.set(Level02);
            }
        }
    }
}

fn stop_level01(
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

    /*
    #[test]
    fn should_test_something() {
        // given
        //let mut _app = App::new();

        // when
        //app.add_event::<HealthDamageReceived>();
        //app.add_systems(Update, damage_received_listener);
        //let entity = app.borrow_mut().world.spawn(Health(100)).id();
        //app.borrow_mut().world.resource_mut::<Events<HealthDamageReceived>>().send(HealthDamageReceived { entity, damage: 10 });
        //app.update();

        // then
        //assert!(app.world.get::<Health>(entity).is_some());
        //assert_eq!(app.world.get::<Health>(entity).unwrap().0, 90);
    }
    */
}
