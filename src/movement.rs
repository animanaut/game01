use bevy::app::Plugin;

use AppState::Running;
use bevy::prelude::*;

use crate::{
    app_states::AppState,
    controls::{Down, Left, PlayerControlled, Right, Up},
    sprites::MoveAnimation,
    tiles::TileCoordinate,
};

// Constants
const NAME: &str = "movement";

// Plugin
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Running), start_movement)
            .add_systems(
                Update,
                (handle_input, update_movement).run_if(in_state(Running)),
            )
            .add_systems(OnExit(Running), stop_movement);
    }
}

// Components

// Resources

// Events

// Systems
fn start_movement(mut _commands: Commands) {
    debug!("starting {}", NAME);
}

fn update_movement() {
    debug!("updating {}", NAME);
}

/// receive input events and trigger movement and animations here.
fn handle_input(
    mut commands: Commands,
    mut players: Query<(Entity, &mut TileCoordinate), With<PlayerControlled>>,
    mut left: EventReader<Left>,
    mut right: EventReader<Right>,
    mut up: EventReader<Up>,
    mut down: EventReader<Down>,
) {
    debug!("handle input {}", NAME);

    for _ in left.read() {
        debug!("handle left input");

        for (e, tc) in players.iter_mut() {
            // TODO: check for movement here
            let start = tc.clone();
            let mut end = tc.clone();
            end.x = tc.x - 1;
            commands.entity(e).insert(MoveAnimation {
                start,
                end,
                ..default()
            });
        }
    }

    for _ in right.read() {
        debug!("handle right input");

        for (e, tc) in players.iter_mut() {
            let start = tc.clone();
            let mut end = tc.clone();
            end.x = tc.x + 1;
            commands.entity(e).insert(MoveAnimation {
                start,
                end,
                ..default()
            });
        }
    }

    for _ in up.read() {
        debug!("handle up input");

        for (e, tc) in players.iter_mut() {
            let start = tc.clone();
            let mut end = tc.clone();
            end.y = tc.y + 1;
            commands.entity(e).insert(MoveAnimation {
                start,
                end,
                ..default()
            });
        }
    }

    for _ in down.read() {
        debug!("handle up input");

        for (e, tc) in players.iter_mut() {
            let start = tc.clone();
            let mut end = tc.clone();
            end.y = tc.y - 1;
            commands.entity(e).insert(MoveAnimation {
                start,
                end,
                ..default()
            });
        }
    }
}

fn stop_movement(mut _commands: Commands) {
    debug!("stopping {}", NAME);
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
