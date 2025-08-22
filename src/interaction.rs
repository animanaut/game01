use bevy::app::Plugin;

use AppState::Running;
use bevy::prelude::*;

use crate::app_states::AppState;

// Constants
const NAME: &str = "interaction";

// Plugin
pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Running), start_interaction)
            .add_systems(Update, (update_interaction).run_if(in_state(Running)))
            .add_systems(OnExit(Running), stop_interaction);
    }
}

// Components

// Resources

// Events

// Systems
fn start_interaction(mut _commands: Commands) {
    debug!("starting {}", NAME);
}

fn update_interaction() {
    debug!("updating {}", NAME);
}

fn stop_interaction(mut _commands: Commands) {
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
