use std::fmt::Display;

use bevy::app::Plugin;

use AppState::Running;
use bevy::prelude::*;

use crate::app_states::AppState;

// Constants
const NAME: &str = "tiles";

// Plugin
pub struct TilesPlugin;

impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Running), start_tiles)
            .add_systems(Update, (logging_tiles).run_if(in_state(Running)))
            .add_systems(OnExit(Running), stop_tiles);
    }
}

// Components
#[derive(Component)]
#[allow(dead_code)]
pub struct Tile;

#[derive(Component, PartialEq, Clone, Default)]
pub struct TileCoordinate {
    pub x: i64,
    pub y: i64,
}

impl Display for TileCoordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TileCoordinate: x: {}, y: {}", self.x, self.y)
    }
}

// Resources

// Events

// Systems
fn start_tiles(mut _commands: Commands) {
    debug!("starting {}", NAME);
}

fn logging_tiles(tile_coordinates: Query<(Entity, &TileCoordinate)>) {
    debug!("logging {}", NAME);
    for (entity, coordinate) in tile_coordinates.iter() {
        debug!("entity: {}, coordinate: {}", entity, coordinate);
    }
}

fn stop_tiles(mut _commands: Commands) {
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
