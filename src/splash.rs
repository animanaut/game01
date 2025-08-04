use bevy::app::Plugin;

use AppState::{MainMenu, Splash};
use bevy::prelude::*;

use crate::app_states::AppState;

// Constants
const NAME: &str = "splash";

// Plugin
pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Splash), start_splash)
            .add_systems(Update, (update_splash).run_if(in_state(Splash)))
            .add_systems(OnExit(Splash), stop_splash);
    }
}

// Components
#[derive(Component)]
struct SplashComponent;

// Resources
#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

// Events

// Systems
fn start_splash(mut commands: Commands, asset_server: Res<AssetServer>) {
    debug!("starting {}", NAME);

    // entities
    commands
        .spawn(Sprite {
            image: asset_server.load("bevy_icon.png"),
            ..Default::default()
        })
        .insert(SplashComponent);

    // resources
    commands.insert_resource(SplashTimer(Timer::from_seconds(2.0, TimerMode::Once)));
}
fn update_splash(
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    debug!("updating {}", NAME);
    if timer.tick(time.delta()).finished() {
        next_state.set(MainMenu);
    }
}
fn stop_splash(mut commands: Commands, to_despawn: Query<Entity, With<SplashComponent>>) {
    debug!("stopping {}", NAME);
    // cleanup

    // entities
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }

    // resources
    commands.remove_resource::<SplashTimer>();
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
