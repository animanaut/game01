use bevy::app::Plugin;
use bevy::prelude::*;

// Constants
const _NAME: &str = "app_states";

// Types
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    _Running,
    _Quitting,
}

// Plugin
pub struct AppStatesPlugin;

impl Plugin for AppStatesPlugin {
    fn build(&self, _app: &mut App) {
        // nothing to do until now
    }
}

// Components

// Resources

// Events

// Systems

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
