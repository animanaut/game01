use std::time::Duration;

use bevy::app::Plugin;

use AppState::Running;
use bevy::prelude::*;

use crate::{
    app_states::AppState,
    controls::{Down, Left, PlayerControlled, Right, Up},
    sprites::{ANIM_DURATION, MoveAnimation, SPRITE_DIM, SPRITE_SCALE},
    tiles::TileCoordinate,
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
                (
                    update_in_game,
                    handle_input,
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

/// receive input events and trigger movement and animations here.
fn handle_input(
    mut commands: Commands,
    mut players: Query<
        (Entity, &mut TileCoordinate, Option<&mut MoveAnimation>),
        With<PlayerControlled>,
    >,
    mut left: EventReader<Left>,
    mut right: EventReader<Right>,
    mut up: EventReader<Up>,
    mut down: EventReader<Down>,
) {
    debug!("handle input {}", NAME);

    for _ in left.read() {
        debug!("handle left input");

        for (e, tc, a) in players.iter_mut() {
            let start = tc.clone();
            let mut end = tc.clone();
            end.x = tc.x - 1;
            if let Some(mut animation) = a {
                animation.timer = Timer::new(Duration::from_millis(ANIM_DURATION), TimerMode::Once);
                animation.start = start;
                animation.end = end;
            } else {
                commands.entity(e).insert(MoveAnimation {
                    start,
                    end,
                    ..default()
                });
            }
        }
    }

    for _ in right.read() {
        debug!("handle right input");

        for (e, tc, a) in players.iter_mut() {
            let start = tc.clone();
            let mut end = tc.clone();
            end.x = tc.x + 1;
            if let Some(mut animation) = a {
                animation.timer = Timer::new(Duration::from_millis(ANIM_DURATION), TimerMode::Once);
                animation.start = start;
                animation.end = end;
            } else {
                commands.entity(e).insert(MoveAnimation {
                    start,
                    end,
                    ..default()
                });
            }
        }
    }

    for _ in up.read() {
        debug!("handle up input");

        for (e, tc, a) in players.iter_mut() {
            let start = tc.clone();
            let mut end = tc.clone();
            end.y = tc.y + 1;
            if let Some(mut animation) = a {
                animation.timer = Timer::new(Duration::from_millis(ANIM_DURATION), TimerMode::Once);
                animation.start = start;
                animation.end = end;
            } else {
                commands.entity(e).insert(MoveAnimation {
                    start,
                    end,
                    ..default()
                });
            }
        }
    }

    for _ in down.read() {
        debug!("handle up input");

        for (e, tc, a) in players.iter_mut() {
            let start = tc.clone();
            let mut end = tc.clone();
            end.y = tc.y - 1;
            if let Some(mut animation) = a {
                animation.timer = Timer::new(Duration::from_millis(ANIM_DURATION), TimerMode::Once);
                animation.start = start;
                animation.end = end;
            } else {
                commands.entity(e).insert(MoveAnimation {
                    start,
                    end,
                    ..default()
                });
            }
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
