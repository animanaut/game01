use bevy::app::Plugin;

use AppState::{MainMenu, Running};
use LevelState::Level02;
use bevy::prelude::*;

use crate::{
    app_states::{AppState, LevelState},
    controls::PlayerControlled,
    sprites::{ExfilSprite, MySprite, SpawnSprite, Tile},
    tiles::TileCoordinate,
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
                (update_level01, check_for_exit_level02)
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
        tile: Tile::Player01,
        color: Some(Color::linear_rgb(0.5, 0.5, 0.5)),
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 1, y: 0, z: 1 },
        tile: Tile::LevelExit01,
        color: Some(Color::linear_rgb(0.0, 0.5, 0.5)),
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 2, y: 1, z: -1 },
        tile: Tile::Grass,
        ..default()
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 1, y: 1, z: -1 },
        tile: Tile::GrassFlowers,
        ..default()
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 0, y: 1, z: -1 },
        tile: Tile::LongGrass,
        ..default()
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 0, y: -2, z: -1 },
        tile: Tile::GoldCoin,
        ..default()
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 2, y: -2, z: -1 },
        tile: Tile::GoldCoins,
        ..default()
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 4, y: -2, z: -1 },
        tile: Tile::GoldCoinBag,
        ..default()
    });
}

fn update_level01() {
    debug!("updating {}", NAME);
}

fn check_for_exit_level02(
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
