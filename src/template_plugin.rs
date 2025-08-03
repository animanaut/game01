use bevy::app::Plugin;

use AppState::MainMenu;
use bevy::prelude::*;

use crate::app_states::AppState;

// Constants
const NAME: &str = "template";

// Plugin
pub struct TemplatePlugin;

impl Plugin for TemplatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainMenu), start_template_system)
            .add_systems(Update, (update_template_system).run_if(in_state(MainMenu)))
            .add_systems(OnExit(MainMenu), stop_template_system);
    }
}

// Components

// Resources

// Events

// Systems
fn start_template_system(mut _commands: Commands) {
    debug!("starting {}", NAME);
}
fn update_template_system() {
    debug!("updating {}", NAME);
}
fn stop_template_system(mut _commands: Commands) {
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
