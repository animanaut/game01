use std::time::Duration;

use bevy::app::Plugin;

use AppState::{MainMenu, Running};
use bevy::prelude::*;

use crate::{
    app_states::AppState,
    controls::{Left, PlayerControlled, Right},
    sprites::{Animation, ExfilSprite, SPRITE_DIM, SPRITE_SCALE},
    tiles::TileCoordinate,
};

// Constants
const NAME: &str = "in_game";
const ANIM_DURATION: u64 = 200;

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
                    check_for_exit,
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
        (Entity, &Transform, &TileCoordinate, Option<&Animation>),
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

fn update_transforms(mut animations: Query<(&mut Transform, &Animation)>) {
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
    mut animation_changes: Query<(&Animation, &mut TileCoordinate), Changed<Animation>>,
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
        (
            Entity,
            &Transform,
            &mut TileCoordinate,
            Option<&mut Animation>,
        ),
        With<PlayerControlled>,
    >,
    mut left: EventReader<Left>,
    mut right: EventReader<Right>,
) {
    debug!("handle input {}", NAME);

    for _ in left.read() {
        debug!("handle left input");

        for (e, t, tc, a) in players.iter_mut() {
            debug!("initial t of animation: {}", t.translation);
            let mut new_end_tile = tc.clone();
            new_end_tile.x = tc.x - 1;
            if let Some(mut animation) = a {
                animation.timer = Timer::new(Duration::from_millis(ANIM_DURATION), TimerMode::Once);
                animation.start = tc.clone();
                animation.end = new_end_tile;
            } else {
                commands.entity(e).insert(Animation {
                    timer: Timer::new(Duration::from_millis(ANIM_DURATION), TimerMode::Once),
                    function: EaseFunction::CircularInOut,
                    start: tc.clone(),
                    end: TileCoordinate {
                        x: tc.x - 1,
                        y: tc.y,
                    },
                });
            }
        }
    }

    for _ in right.read() {
        debug!("handle right input");

        for (e, t, tc, a) in players.iter_mut() {
            debug!("initial t of animation: {}", t.translation);
            let mut new_end_tile = tc.clone();
            new_end_tile.x = tc.x + 1;
            if let Some(mut animation) = a {
                animation.timer = Timer::new(Duration::from_millis(ANIM_DURATION), TimerMode::Once);
                animation.start = tc.clone();
                animation.end = new_end_tile;
            } else {
                commands.entity(e).insert(Animation {
                    timer: Timer::new(Duration::from_millis(ANIM_DURATION), TimerMode::Once),
                    function: EaseFunction::CircularInOut,
                    start: tc.clone(),
                    end: TileCoordinate {
                        x: tc.x + 1,
                        y: tc.y,
                    },
                });
            }
        }
    }
}

fn check_for_exit(
    mut next_state: ResMut<NextState<AppState>>,
    players: Query<&TileCoordinate, (With<PlayerControlled>, Without<ExfilSprite>)>,
    exfils: Query<&TileCoordinate, (With<ExfilSprite>, Without<PlayerControlled>)>,
) {
    debug!("checking exit {}", NAME);
    if let Ok(player_coordinate) = players.single() {
        for exfil_coordinate in exfils.iter() {
            if player_coordinate.eq(exfil_coordinate) {
                // TODO: smoother transition, maybe with animation on an event
                next_state.set(MainMenu);
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
