use bevy::app::Plugin;

use AppState::Running;
use LevelState::Level05;
use bevy::prelude::*;

use crate::app_states::AppState;
use crate::app_states::LevelState;

// Constants
const NAME: &str = "level05";

// Plugin
pub struct Level05Plugin;

impl Plugin for Level05Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Level05), start_level05)
            .add_systems(
                Update,
                (update_level05)
                    .run_if(in_state(Running))
                    .run_if(in_state(Level05)),
            )
            .add_systems(OnExit(Level05), stop_level05);
    }
}

// Components

// Resources

// Events

// Systems
fn start_level05(mut _commands: Commands) {
    debug!("starting {}", NAME);
}

fn update_level05() {
    debug!("updating {}", NAME);
}

fn stop_level05(mut _commands: Commands) {
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
