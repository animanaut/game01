use bevy::{app::Plugin, platform::collections::HashMap};

use AppState::Running;
use bevy::prelude::*;

use crate::{
    app_states::AppState,
    controls::{Down, Left, PlayerControlled, Right, Up},
    in_game::LevelFinished,
    sprites::MoveAnimation,
    tiles::{SolidTile, TileCoordinate},
};

// Constants
const NAME: &str = "movement";

// Plugin
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            // events
            .add_event::<MoveBlocked>()
            .add_event::<Interacted>()
            // systems
            .add_systems(OnEnter(Running), start_movement)
            .add_systems(
                Update,
                (handle_input, update_movement, solid_tiles_added).run_if(in_state(Running)),
            )
            .add_systems(
                Update,
                (reset_movement)
                    .run_if(resource_exists::<SolidTiles>)
                    .run_if(on_event::<LevelFinished>),
            )
            .add_systems(
                OnExit(Running),
                stop_movement.run_if(resource_exists::<SolidTiles>),
            );
    }
}

// Components

// Resources
#[derive(Resource, Default, Debug)]
struct SolidTiles {
    pub map: HashMap<TileCoordinate, Entity>,
}

// Events
#[derive(Event)]
#[allow(dead_code)]
pub struct PlayerMoved;

/// player move blocked by an entity
#[derive(Event)]
#[allow(dead_code)]
pub struct MoveBlocked {
    pub mover: Entity,
    pub blocked_by: Entity,
}

/// player interacted with an entity
#[derive(Event)]
#[allow(dead_code)]
pub struct Interacted {
    pub triggered_by: Entity,
    pub interacted_with: Entity,
}

// Systems
fn start_movement(mut commands: Commands) {
    debug!("starting {}", NAME);
    commands.insert_resource(SolidTiles::default());
}

fn update_movement() {
    debug!("updating {}", NAME);
}

fn solid_tiles_added(
    mut _commands: Commands,
    added: Query<(Entity, &TileCoordinate), Added<SolidTile>>,
    mut solid_tiles: ResMut<SolidTiles>,
) {
    for (entity, tile_coordinate) in added.iter() {
        debug!("solid tiles added at {} for {}", tile_coordinate, entity);
        if let Some(overridden) = solid_tiles.map.insert(tile_coordinate.clone(), entity) {
            debug!(
                "solid tiles replaced at {} for {}. old entity: {}",
                tile_coordinate, entity, overridden
            );
            // TODO: cleanup overridden solid tile. not sure how if this is the right place
            // TODO: following line will crash CountdownTimer event listener
            //commands.entity(overridden).despawn();
        }
    }
}

/// receive input events and trigger movement and animations here.
fn handle_input(
    mut commands: Commands,
    mut players: Query<(Entity, &mut TileCoordinate), With<PlayerControlled>>,
    mut left: EventReader<Left>,
    mut right: EventReader<Right>,
    mut up: EventReader<Up>,
    mut down: EventReader<Down>,
    solid_blocks: Res<SolidTiles>,
    mut blocked: EventWriter<MoveBlocked>,
    mut _interacted: EventWriter<Interacted>,
) {
    debug!("handle input {}", NAME);

    for _ in left.read() {
        debug!("handle left input");

        for (mover, tc) in players.iter_mut() {
            let start = tc.clone();
            let mut end = tc.clone();
            end.x = tc.x - 1;

            if let Some(blocked_by) = solid_blocks.map.get(&end) {
                blocked.write(MoveBlocked {
                    mover,
                    blocked_by: *blocked_by,
                });
            } else {
                commands.entity(mover).insert(MoveAnimation {
                    start,
                    end,
                    ..default()
                });
            }
        }
    }

    for _ in right.read() {
        debug!("handle right input");

        for (e, tc) in players.iter_mut() {
            let start = tc.clone();
            let mut end = tc.clone();
            end.x = tc.x + 1;

            if solid_blocks.map.get(&end).is_none() {
                commands.entity(e).insert(MoveAnimation {
                    start,
                    end,
                    ..default()
                });
            }
        }
    }

    for _ in up.read() {
        debug!("handle up input");

        for (e, tc) in players.iter_mut() {
            let start = tc.clone();
            let mut end = tc.clone();
            end.y = tc.y + 1;

            if solid_blocks.map.get(&end).is_none() {
                commands.entity(e).insert(MoveAnimation {
                    start,
                    end,
                    ..default()
                });
            }
        }
    }

    for _ in down.read() {
        debug!("handle up input");

        for (e, tc) in players.iter_mut() {
            let start = tc.clone();
            let mut end = tc.clone();
            end.y = tc.y - 1;

            if solid_blocks.map.get(&end).is_none() {
                commands.entity(e).insert(MoveAnimation {
                    start,
                    end,
                    ..default()
                });
            }
        }
    }
}

fn reset_movement(mut commands: Commands, mut events: EventReader<LevelFinished>) {
    for _ in events.read() {
        commands.insert_resource(SolidTiles::default());
    }
}

fn stop_movement(mut commands: Commands) {
    debug!("stopping {}", NAME);
    commands.remove_resource::<SolidTiles>();
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
