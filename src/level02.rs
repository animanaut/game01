use std::time::Duration;

use bevy::app::Plugin;

use AppState::Running;
use LevelState::{Level02, Level03};
use bevy::prelude::*;

use crate::{
    animation::{Animation, AnimationType},
    app_states::{AppState, LevelState},
    controls::PlayerControlled,
    gold::PlayerPickedUpGoldCoins,
    sprites::{ExfilSprite, MySprite, SpawnSprite, SpriteSheetTile},
    tiles::TileCoordinate,
    tutorial::Tutorial,
};

// Constants
const NAME: &str = "level02";

// Plugin
pub struct Level02Plugin;

impl Plugin for Level02Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Level02), start_level02)
            .add_systems(
                Update,
                (
                    update_level02,
                    added_tutorial_components,
                    check_for_tutorial_action,
                    check_for_exit_level02,
                )
                    .run_if(in_state(Running))
                    .run_if(in_state(Level02)),
            )
            .add_systems(OnExit(Level02), stop_level02);
    }
}

// Components

// Resources

// Events

// Systems
fn start_level02(mut spawn_sprite: EventWriter<SpawnSprite>) {
    debug!("starting {}", NAME);

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 0, y: 0, z: 0 },
        tile: SpriteSheetTile::Player01,
        color: Some(Color::linear_rgb(0.5, 0.5, 0.5)),
        ..default()
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 1, y: 0, z: 1 },
        tile: SpriteSheetTile::LevelExit01,
        color: Some(Color::linear_rgb(0.0, 0.5, 0.5)),
        ..default()
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 2, y: 1, z: -1 },
        tile: SpriteSheetTile::Grass,
        ..default()
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 1, y: 1, z: -1 },
        tile: SpriteSheetTile::GrassFlowers,
        ..default()
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 0, y: 1, z: -1 },
        tile: SpriteSheetTile::LongGrass,
        ..default()
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 0, y: -2, z: -1 },
        tile: SpriteSheetTile::GoldCoin,
        tutorial: true,
        ..default()
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 2, y: -2, z: -1 },
        tile: SpriteSheetTile::GoldCoins,
        tutorial: true,
        ..default()
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 4, y: -2, z: -1 },
        tile: SpriteSheetTile::GoldCoinBag,
        tutorial: true,
        ..default()
    });
}

fn update_level02() {
    debug!("updating {}", NAME);
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

fn check_for_tutorial_action(
    mut commands: Commands,
    tutorial: Query<(Entity, &Animation, &AnimationType), With<Tutorial>>,
    mut gold_pickup: EventReader<PlayerPickedUpGoldCoins>,
) {
    for _ in gold_pickup.read() {
        for (t, _, _) in tutorial.iter() {
            commands.entity(t).remove::<Tutorial>();
            commands.entity(t).remove::<Animation>();
            commands.entity(t).remove::<AnimationType>();
        }
    }
}

fn check_for_exit_level02(
    mut next_state: ResMut<NextState<LevelState>>,
    players: Query<&TileCoordinate, (With<PlayerControlled>, Without<ExfilSprite>)>,
    exfils: Query<&TileCoordinate, (With<ExfilSprite>, Without<PlayerControlled>)>,
) {
    debug!("checking exit {}", NAME);
    for player_coordinate in players.iter() {
        for exfil_coordinate in exfils.iter() {
            if player_coordinate.eq2d(exfil_coordinate) {
                // TODO: smoother transition, maybe with animation on an event
                next_state.set(Level03);
            }
        }
    }
}

fn stop_level02(mut commands: Commands, sprites: Query<Entity, With<MySprite>>) {
    debug!("stopping {}", NAME);
    for sprite in sprites.iter() {
        commands.entity(sprite).despawn();
    }
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
