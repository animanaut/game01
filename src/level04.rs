use std::time::Duration;

use bevy::app::Plugin;

use AppState::{MainMenu, Running};
use LevelState::Level04;
use bevy::prelude::*;

use crate::{
    animation::{Animation, AnimationType},
    app_states::{AppState, LevelState},
    controls::{Down, Left, PlayerControlled, Right, Up},
    in_game::LevelFinished,
    interaction::{Interacted, InteractionId, InteractionSource, InteractionTarget},
    sprites::{ExfilSprite, MySprite, SpawnSprite, SpriteSheetTile},
    tiles::{DoorTile, InteractableTile, TileCoordinate, TriggerTile},
    tutorial::{CountDownFinished, CountDownTutorialCounter, Tutorial, TutorialCountdown},
};

// Constants
const NAME: &str = "level04";

// Plugin
pub struct Level04Plugin;

impl Plugin for Level04Plugin {
    fn build(&self, app: &mut App) {
        app
            // events
            // ...
            // systems
            .add_systems(OnEnter(Level04), start_level04)
            .add_systems(
                Update,
                (
                    update_level04,
                    added_tutorial_components,
                    added_interaction_components,
                    countdown_tutorial,
                    check_for_exit_level04,
                )
                    .run_if(in_state(Running))
                    .run_if(in_state(Level04)),
            )
            .add_systems(
                Update,
                (countdown_tutorial_finished)
                    .run_if(on_event::<CountDownFinished>)
                    .run_if(in_state(Running))
                    .run_if(in_state(Level04)),
            )
            .add_systems(
                Update,
                (interacted)
                    .run_if(on_event::<Interacted>)
                    .run_if(in_state(Running))
                    .run_if(in_state(Level04)),
            )
            .add_systems(OnExit(Level04), stop_level04);
    }
}

// Components

// Resources

// Events

// Systems
fn start_level04(mut spawn_sprite: EventWriter<SpawnSprite>) {
    debug!("starting {}", NAME);

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 0, y: 0, z: 0 },
        tile: SpriteSheetTile::Player01,
        color: Some(Color::linear_rgb(0.5, 0.5, 0.5)),
        ..default()
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 2, y: 1, z: 0 },
        tile: SpriteSheetTile::MechanicDoor,
        color: Some(Color::linear_rgb(0.0, 0.5, 0.5)),
        tutorial: true,
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: -2, y: 1, z: 0 },
        tile: SpriteSheetTile::BottomLeverLeft,
        tutorial: true,
        ..default()
    });
}

fn update_level04() {
    debug!("updating {}", NAME);
}

/// nothing fancy, we just have a lever and a door in level04
fn interacted(
    mut commands: Commands,
    mut interacted: EventReader<Interacted>,
    interactables: Query<(Entity, &InteractableTile)>,
    mut spawn_sprite: EventWriter<SpawnSprite>,
) {
    debug!("interacted {}", NAME);
    for i in interacted.read() {
        debug!("interacted {}: have message", NAME);
        if let Ok((entity, _)) = interactables.get(i.0) {
            debug!("interacted {}: found target", NAME);
            commands.entity(entity).despawn();
            spawn_sprite.write(SpawnSprite {
                coordinate: TileCoordinate { x: 2, y: 1, z: 0 },
                tile: SpriteSheetTile::LevelExit01,
                color: Some(Color::linear_rgb(0.0, 0.5, 0.5)),
                ..default()
            });
        }
    }
}

fn added_interaction_components(
    mut commands: Commands,
    added_doors: Query<Entity, Added<DoorTile>>,
    added_trigger: Query<Entity, Added<TriggerTile>>,
) {
    debug!("added interaction {}", NAME);
    // simple level: one door, one lever, one id
    let id = InteractionId(123);
    if let Ok(door) = added_doors.single() {
        debug!("added interaction id {:?} to door {}", id, door);
        commands.entity(door).insert(InteractionTarget(id.clone()));
    }
    if let Ok(lever) = added_trigger.single() {
        debug!("added interaction id {:?} to lever {}", id, lever);
        commands.entity(lever).insert(InteractionSource(id));
    }
}

fn added_tutorial_components(
    mut commands: Commands,
    added_tutorials: Query<Entity, Added<Tutorial>>,
) {
    for added in added_tutorials.iter() {
        commands.entity(added).insert(TutorialCountdown::new(3));
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
    mut tutorial_countdown: EventWriter<CountDownTutorialCounter>,
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
            tutorial_countdown.write(CountDownTutorialCounter(c));
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

fn countdown_tutorial_finished(
    mut commands: Commands,
    mut finished: EventReader<CountDownFinished>,
) {
    debug!("countdown finished {}", NAME);
    for f in finished.read() {
        debug!("removing tutorial components for {}", f.0);
        commands.entity(f.0).remove::<Tutorial>();
        commands.entity(f.0).remove::<TutorialCountdown>();
    }
}

fn check_for_exit_level04(
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

fn stop_level04(
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
