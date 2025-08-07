use std::time::Duration;

use bevy::app::Plugin;

use AppState::{MainMenu, Running};
use bevy::prelude::*;

use crate::{
    app_states::AppState,
    controls::{Left, PlayerControlled, Right},
    sprites::{Animation, ExfilSprite, SPRITE_DIM, SPRITE_SCALE},
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
                    handle_animation,
                    check_for_exit,
                    log_transforms,
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

fn log_transforms(transforms: Query<(&Transform, Option<&Animation>), With<PlayerControlled>>) {
    for (_t, a) in transforms.iter() {
        if let Some(a) = a {
            debug!("logging animation: {}", a);
        }
    }
}

fn handle_animation(mut animations: Query<(&mut Transform, &Animation)>) {
    debug!("handling animation {}", NAME);
    for (mut t, a) in animations.iter_mut() {
        let direction = a.end.translation - a.start.translation;
        let eased_fraction = a.function.sample(a.timer.fraction());
        if let Some(fraction) = eased_fraction {
            t.translation = a.start.translation + direction * fraction;
        }
    }
}

fn handle_input(
    mut commands: Commands,
    mut players: Query<(Entity, &Transform, Option<&mut Animation>), With<PlayerControlled>>,
    mut left: EventReader<Left>,
    mut right: EventReader<Right>,
) {
    debug!("handle input {}", NAME);

    for _ in left.read() {
        debug!("handle left input");

        for (e, t, a) in players.iter_mut() {
            debug!("initial t of animation: {}", t.translation);
            let new_x = t.translation.x - SPRITE_SCALE * SPRITE_DIM as f32;
            let mut new_end = *t;
            new_end.translation.x = new_x;
            if let Some(mut animation) = a {
                animation.timer = Timer::new(Duration::from_millis(ANIM_DURATION), TimerMode::Once);
                animation.start = *t;
                animation.end = new_end;
            } else {
                commands.entity(e).insert(Animation {
                    timer: Timer::new(Duration::from_millis(ANIM_DURATION), TimerMode::Once),
                    function: EaseFunction::CircularInOut,
                    start: *t,
                    end: new_end,
                });
            }
        }
    }

    for _ in right.read() {
        debug!("handle right input");

        for (e, t, a) in players.iter_mut() {
            debug!("initial t of animation: {}", t.translation);
            let new_x = t.translation.x + SPRITE_SCALE * SPRITE_DIM as f32;
            let mut new_end = *t;
            new_end.translation.x = new_x;
            if let Some(mut animation) = a {
                animation.timer = Timer::new(Duration::from_millis(ANIM_DURATION), TimerMode::Once);
                animation.start = *t;
                animation.end = new_end;
            } else {
                commands.entity(e).insert(Animation {
                    timer: Timer::new(Duration::from_millis(ANIM_DURATION), TimerMode::Once),
                    function: EaseFunction::CircularInOut,
                    start: *t,
                    end: new_end,
                });
            }
        }
    }
}

fn check_for_exit(
    mut next_state: ResMut<NextState<AppState>>,
    players: Query<&mut Transform, (With<PlayerControlled>, Without<ExfilSprite>)>,
    exfils: Query<&mut Transform, (With<ExfilSprite>, Without<PlayerControlled>)>,
) {
    debug!("checking exit {}", NAME);
    if let Ok(player_transform) = players.single() {
        for exfil_transform in exfils.iter() {
            if player_transform.eq(exfil_transform) {
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
