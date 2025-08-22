use bevy::{app::Plugin, platform::collections::HashMap};

use AppState::Running;
use bevy::prelude::*;

use crate::{
    app_states::AppState,
    controls::{Down, Left, PlayerControlled, Right, Up},
    in_game::LevelFinished,
    sprites::MoveAnimation,
    tiles::{InteractableTile, SolidTile, TileCoordinate},
};

// Constants
const NAME: &str = "movement";

// Plugin
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            // events
            .add_event::<MoveTriggered>()
            .add_event::<MoveBlocked>()
            .add_event::<InteractionTriggered>()
            // systems
            .add_systems(OnEnter(Running), start_movement)
            .add_systems(
                Update,
                (
                    handle_input,
                    update_movement,
                    solid_tiles_added,
                    solid_tiles_removed,
                    interaction_tiles_added,
                    interaction_tiles_removed,
                )
                    .run_if(in_state(Running)),
            )
            .add_systems(
                Update,
                (reset_movement)
                    .run_if(resource_exists::<SolidTiles>)
                    .run_if(resource_exists::<InteractionTiles>)
                    .run_if(on_event::<LevelFinished>),
            )
            .add_systems(OnExit(Running), stop_movement);
    }
}

// Components

// Resources
#[derive(Resource, Default, Debug)]
struct SolidTiles {
    pub map: HashMap<TileCoordinate, Entity>,
}

#[derive(Resource, Default, Debug)]
struct InteractionTiles {
    pub map: HashMap<TileCoordinate, Entity>,
}

// Events
#[derive(Event)]
#[allow(dead_code)]
pub struct MoveTriggered {
    pub mover: Entity,
    pub start: TileCoordinate,
    pub end: TileCoordinate,
}

/// player move blocked by an entity
#[allow(dead_code)]
#[derive(Event)]
pub struct MoveBlocked {
    pub mover: Entity,
    pub blocked_by: Entity,
}

/// player interacted with an entity
#[derive(Event)]
#[allow(dead_code)]
pub struct InteractionTriggered {
    pub triggered_by: Entity,
    pub interacted_with: Entity,
}

// Systems
fn start_movement(mut commands: Commands) {
    debug!("starting {}", NAME);
    commands.insert_resource(SolidTiles::default());
    commands.insert_resource(InteractionTiles::default());
}

fn update_movement() {
    debug!("updating {}", NAME);
}

fn solid_tiles_added(
    mut _commands: Commands,
    added: Query<(Entity, &TileCoordinate), Added<SolidTile>>,
    mut solid_tiles: ResMut<SolidTiles>,
) {
    debug!("solid tiles added {}", NAME);
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

fn solid_tiles_removed(
    mut _commands: Commands,
    mut removed: RemovedComponents<SolidTile>,
    mut solid_tiles: ResMut<SolidTiles>,
) {
    debug!("solid tiles removed {}", NAME);
    for entity in removed.read() {
        solid_tiles.map.retain(|_, e| entity.ne(e));
    }
}

fn interaction_tiles_removed(
    mut _commands: Commands,
    mut removed: RemovedComponents<InteractableTile>,
    mut interaction_tiles: ResMut<InteractionTiles>,
) {
    debug!("interaction tiles removed {}", NAME);
    for entity in removed.read() {
        interaction_tiles.map.retain(|_, e| entity.ne(e));
    }
}

fn interaction_tiles_added(
    mut _commands: Commands,
    added: Query<(Entity, &TileCoordinate), Added<InteractableTile>>,
    mut interactables: ResMut<InteractionTiles>,
) {
    debug!("interaction tiles added {}", NAME);
    for (entity, tile_coordinate) in added.iter() {
        debug!(
            "interaction tiles added at {} for {}",
            tile_coordinate, entity
        );
        if let Some(overridden) = interactables.map.insert(tile_coordinate.clone(), entity) {
            debug!(
                "interaction tiles replaced at {} for {}. old entity: {}",
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
    mut moved: EventWriter<MoveTriggered>,
    mut blocked: EventWriter<MoveBlocked>,
    mut interacted: EventWriter<InteractionTriggered>,
    interaction_tiles: Res<InteractionTiles>,
) {
    debug!("handle input {}", NAME);

    for _ in left.read() {
        debug!("handle left input");

        for (mover, tc) in players.iter_mut() {
            let start = tc.clone();
            let mut end = tc.clone();
            end.x = tc.x - 1;

            if let Some(i) = interaction_tiles.map.get(&end) {
                debug!("handle left interaction input: {:?}", interaction_tiles.map);
                interacted.write(InteractionTriggered {
                    triggered_by: mover,
                    interacted_with: *i,
                });
                debug!("handle left interaction input: event sent");
            }

            if let Some(b) = solid_blocks.map.get(&end) {
                debug!("handle left blocking input: {:?}", solid_blocks.map);
                blocked.write(MoveBlocked {
                    mover,
                    blocked_by: *b,
                });
                debug!("handle left blocking input: event sent");
            } else {
                commands.entity(mover).insert(MoveAnimation {
                    start: start.clone(),
                    end: end.clone(),
                    ..default()
                });
                moved.write(MoveTriggered { mover, start, end });
            }
        }
    }

    for _ in right.read() {
        debug!("handle right input");

        for (mover, tc) in players.iter_mut() {
            let start = tc.clone();
            let mut end = tc.clone();
            end.x = tc.x + 1;

            if let Some(i) = interaction_tiles.map.get(&end) {
                interacted.write(InteractionTriggered {
                    triggered_by: mover,
                    interacted_with: *i,
                });
            }

            if let Some(blocked_by) = solid_blocks.map.get(&end) {
                blocked.write(MoveBlocked {
                    mover,
                    blocked_by: *blocked_by,
                });
            } else {
                commands.entity(mover).insert(MoveAnimation {
                    start: start.clone(),
                    end: end.clone(),
                    ..default()
                });
                moved.write(MoveTriggered { mover, start, end });
            }
        }
    }

    for _ in up.read() {
        debug!("handle up input");

        for (mover, tc) in players.iter_mut() {
            let start = tc.clone();
            let mut end = tc.clone();
            end.y = tc.y + 1;

            if let Some(i) = interaction_tiles.map.get(&end) {
                interacted.write(InteractionTriggered {
                    triggered_by: mover,
                    interacted_with: *i,
                });
            }

            if let Some(blocked_by) = solid_blocks.map.get(&end) {
                blocked.write(MoveBlocked {
                    mover,
                    blocked_by: *blocked_by,
                });
            } else {
                commands.entity(mover).insert(MoveAnimation {
                    start: start.clone(),
                    end: end.clone(),
                    ..default()
                });
                moved.write(MoveTriggered { mover, start, end });
            }
        }
    }

    for _ in down.read() {
        debug!("handle up input");

        for (mover, tc) in players.iter_mut() {
            let start = tc.clone();
            let mut end = tc.clone();
            end.y = tc.y - 1;

            if let Some(i) = interaction_tiles.map.get(&end) {
                interacted.write(InteractionTriggered {
                    triggered_by: mover,
                    interacted_with: *i,
                });
            }

            if let Some(blocked_by) = solid_blocks.map.get(&end) {
                blocked.write(MoveBlocked {
                    mover,
                    blocked_by: *blocked_by,
                });
            } else {
                commands.entity(mover).insert(MoveAnimation {
                    start: start.clone(),
                    end: end.clone(),
                    ..default()
                });
                moved.write(MoveTriggered { mover, start, end });
            }
        }
    }
}

fn reset_movement(mut commands: Commands, mut events: EventReader<LevelFinished>) {
    for _ in events.read() {
        commands.insert_resource(SolidTiles::default());
        commands.insert_resource(InteractionTiles::default());
    }
}

fn stop_movement(mut commands: Commands) {
    debug!("stopping {}", NAME);
    commands.remove_resource::<SolidTiles>();
    commands.remove_resource::<InteractionTiles>();
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
