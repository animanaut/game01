use bevy::app::Plugin;

use crate::{app_states::AppState, controls::PlayerControlled};
use AppState::Running;
use bevy::prelude::*;

// Constants
const NAME: &str = "game_camera";

// Plugin
pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Running), start_game_camera)
            .add_systems(Update, (follow_player).run_if(in_state(Running)))
            .add_systems(OnExit(Running), stop_game_camera);
    }
}

// Components
#[derive(Component)]
pub struct GameCamera;

// Resources

// Events

// Systems
fn start_game_camera(mut commands: Commands, cameras: Query<Entity, With<Camera2d>>) {
    debug!("starting {}", NAME);
    if let Ok(camera) = cameras.single() {
        commands.entity(camera).insert(GameCamera);
    }
}

fn follow_player(
    players: Query<&GlobalTransform, (With<PlayerControlled>, Without<GameCamera>)>,
    mut cameras: Query<&mut Transform, (With<GameCamera>, Without<PlayerControlled>)>,
) {
    debug!("updating {}", NAME);
    // TODO: assumption: one player, one camera, for now
    // otherwise we would have to calculate an average and a zoom level to capture all player
    // controlled entities
    if let Ok(player) = players.single() {
        if let Ok(mut camera) = cameras.single_mut() {
            let move_dir = player.translation() - camera.translation;
            camera.translation.x += move_dir.x * 0.125;
            camera.translation.y += move_dir.y * 0.125;
        }
    }
}

fn stop_game_camera(mut commands: Commands, cameras: Query<Entity, With<GameCamera>>) {
    debug!("stopping {}", NAME);
    if let Ok(camera) = cameras.single() {
        commands.entity(camera).remove::<GameCamera>();
    }
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
