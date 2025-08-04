use bevy::app::Plugin;

use AppState::Running;
use bevy::prelude::*;

use crate::{
    app_states::AppState,
    controls::{Left, Right},
};

// Constants
const NAME: &str = "keyboard";

// Plugin
pub struct KeyboardControllerPlugin;

impl Plugin for KeyboardControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Running), start_keyboard_controls)
            .add_systems(Update, (update_keyboard_controls).run_if(in_state(Running)))
            .add_systems(OnExit(Running), stop_keyboard_controls);
    }
}

// Components

// Resources

// Events

// Systems
fn start_keyboard_controls(mut _commands: Commands) {
    debug!("starting {}", NAME);
}

fn update_keyboard_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut left: EventWriter<Left>,
    mut right: EventWriter<Right>,
) {
    debug!("updating {}", NAME);

    if keyboard_input.pressed(KeyCode::KeyA) {
        debug!("sending left event");
        left.write(Left);
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        debug!("sending right event");
        right.write(Right);
    }
}

fn stop_keyboard_controls(mut _commands: Commands) {
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
