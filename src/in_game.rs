use bevy::app::Plugin;

use AppState::Running;
use bevy::prelude::*;

use crate::{
    app_states::AppState,
    controls::{Left, PlayerControlled, Right},
    sprites::{SPRITE_DIM, SPRITE_SCALE},
};

// Constants
const NAME: &str = "in_game";

// Plugin
pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Running), start_in_game)
            .add_systems(
                Update,
                (update_in_game, handle_input).run_if(in_state(Running)),
            )
            .add_systems(OnExit(Running), stop_in_game);
    }
}

// Components

// Resources

// Events

// Systems
fn start_in_game(mut _commands: Commands) {
    debug!("starting {}", NAME);
}

fn update_in_game() {
    debug!("updating {}", NAME);
}

fn handle_input(
    mut players: Query<&mut Transform, With<PlayerControlled>>,
    mut left: EventReader<Left>,
    mut right: EventReader<Right>,
) {
    debug!("handle input {}", NAME);

    for _ in left.read() {
        debug!("handle left input");

        for mut t in players.iter_mut() {
            t.translation.x -= SPRITE_SCALE * SPRITE_DIM as f32;
        }
    }

    for _ in right.read() {
        debug!("handle right input");

        for mut t in players.iter_mut() {
            t.translation.x += SPRITE_SCALE * SPRITE_DIM as f32;
        }
    }
}

fn stop_in_game(mut _commands: Commands) {
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
