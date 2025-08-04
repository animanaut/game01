use bevy::app::Plugin;

use AppState::Running;
use bevy::prelude::*;

use crate::app_states::AppState;

// Constants
const NAME: &str = "controls";

// Plugin
pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Events
            .add_event::<Right>()
            // Systems
            .add_systems(OnEnter(Running), start_controls)
            .add_systems(Update, (update_controls).run_if(in_state(Running)))
            .add_systems(OnExit(Running), stop_controls);
    }
}

// Components
#[derive(Component)]
pub struct PlayerControlled;

// Resources

// Events
#[derive(Event)]
pub struct Right;

// Systems
fn start_controls(mut _commands: Commands) {
    debug!("starting {}", NAME);
}

fn update_controls(mut right: EventReader<Right>) {
    debug!("updating {}", NAME);

    for _ in right.read() {
        debug!("received right event");
    }
}

fn stop_controls(mut _commands: Commands) {
    debug!("stopping {}", NAME);
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
