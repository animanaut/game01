use bevy::app::Plugin;

use AppState::Running;
use LevelState::Level01;
use LevelState::Level02;
use bevy::prelude::*;

use crate::{
    app_states::{AppState, LevelState},
    controls::PlayerControlled,
    sprites::{ExfilSprite, MySprite, SpawnSprite, Tile},
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
                (update_level01, check_for_exit_level01)
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
        coordinate: TileCoordinate { x: 0, y: 0 },
        tile: Tile::Player01,
        color: None,
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 2, y: 0 },
        tile: Tile::LevelExit01,
        color: None,
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 2, y: 1 },
        tile: Tile::Grass,
        color: None,
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 1, y: 1 },
        tile: Tile::GrassFlowers,
        color: None,
    });

    spawn_sprite.write(SpawnSprite {
        coordinate: TileCoordinate { x: 0, y: 1 },
        tile: Tile::LongGrass,
        color: None,
    });
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
            if player_coordinate.eq(exfil_coordinate) {
                // TODO: smoother transition, maybe with animation on an event
                debug!("changing LevelState to {:?}", Level02);
                next_state.set(Level02);
            }
        }
    }
}

fn stop_level01(mut commands: Commands, sprites: Query<Entity, With<MySprite>>) {
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
