use bevy::app::Plugin;

use bevy::prelude::*;

use AppState::Running;

use crate::{
    app_states::AppState,
    controls::PlayerControlled,
    sprites::{MoveAnimation, SPRITE_DIM, SPRITE_SCALE},
    tiles::TileCoordinate,
};

// Constants
const NAME: &str = "in_game";

// Plugin
pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app
            // events
            .add_event::<LevelStarted>()
            .add_event::<LevelFinished>()
            // systems
            .add_systems(OnEnter(Running), start_in_game)
            .add_systems(
                Update,
                (
                    update_in_game,
                    update_tile_coordinates,
                    update_transforms,
                    logging,
                )
                    .run_if(in_state(Running)),
            )
            .add_systems(OnExit(Running), stop_in_game);
    }
}

// Components

// Resources

// Events
#[derive(Event)]
pub struct LevelStarted;

/// event when level is done
#[derive(Event)]
pub struct LevelFinished;

// Systems
fn start_in_game(mut _commands: Commands) {
    debug!("starting {}", NAME);
}

fn update_in_game() {
    debug!("updating {}", NAME);
}

fn logging(
    transforms: Query<
        (Entity, &Transform, &TileCoordinate, Option<&MoveAnimation>),
        With<PlayerControlled>,
    >,
) {
    for (e, t, tc, a) in transforms.iter() {
        if let Some(a) = a {
            debug!("logging animation for {}: {}", e, a);
            debug!("logging tile_coordinate for {}: {}", e, tc);
            debug!(
                "logging translation for {}: transform: {}",
                e, t.translation
            );
        }
    }
}

fn update_transforms(mut animations: Query<(&mut Transform, &MoveAnimation)>) {
    debug!("handling animation {}", NAME);

    for (mut t, a) in animations.iter_mut() {
        let start_t = Transform::from_scale(Vec3::splat(SPRITE_SCALE)).with_translation(Vec3::new(
            SPRITE_SCALE * a.start.x as f32 * SPRITE_DIM as f32,
            SPRITE_SCALE * a.start.y as f32 * SPRITE_DIM as f32,
            0.0,
        ));

        let end_t = Transform::from_scale(Vec3::splat(SPRITE_SCALE)).with_translation(Vec3::new(
            SPRITE_SCALE * a.end.x as f32 * SPRITE_DIM as f32,
            SPRITE_SCALE * a.end.y as f32 * SPRITE_DIM as f32,
            0.0,
        ));
        let direction = end_t.translation - start_t.translation;
        let eased_fraction = a.function.sample(a.timer.fraction());
        if let Some(fraction) = eased_fraction {
            t.translation = start_t.translation + direction * fraction;
        }
    }
}

fn update_tile_coordinates(
    mut animation_changes: Query<(&MoveAnimation, &mut TileCoordinate), Changed<MoveAnimation>>,
) {
    for (a, mut tc) in animation_changes.iter_mut() {
        // TODO: just assign end_tile? throws a cannot be dereferenced currently
        tc.x = a.end.x;
        tc.y = a.end.y;
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
