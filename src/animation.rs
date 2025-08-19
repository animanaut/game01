use std::{fmt::Display, time::Duration};

use bevy::app::Plugin;

use AppState::Running;
use bevy::prelude::*;

use crate::app_states::AppState;

// Constants
const NAME: &str = "animation";

pub const ANIM_DURATION: u64 = 200;

// Plugin
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            // events
            .add_event::<AnimationFinished>()
            // systems
            .add_systems(
                Update,
                (update_animation_timer, update_animation, cleanup_animations)
                    .run_if(in_state(Running)),
            );
    }
}

// Components

/// for now limit these to rotation and scaling as moving has its own component
#[derive(Component, Default)]
#[allow(dead_code)]
pub enum AnimationType {
    #[default]
    /// rotates left/right
    Wiggle,
    /// scale around 1.0
    Pulse,
    /// scale from 0.0 to 1.0, also fade in
    Popup,
    /// blow up from 1.0, fade out
    Burst,
}

#[derive(Component)]
pub struct Animation {
    timer: Timer,
    function: EaseFunction,
}

impl Animation {
    pub fn new(timer: Timer, function: EaseFunction) -> Self {
        Animation { timer, function }
    }

    pub fn eased_fraction(&self) -> f32 {
        let eased_fraction = self.function.sample(self.timer.fraction());
        eased_fraction.unwrap_or(1.0)
    }
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_millis(ANIM_DURATION), TimerMode::Once),
            function: EaseFunction::CircularInOut,
        }
    }
}

impl Display for Animation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Animation: timer fraction: {}, timer remaining: {}",
            self.timer.fraction(),
            self.timer.remaining_secs()
        )
    }
}

// Resources

// Events
#[derive(Event)]
pub struct AnimationFinished(Entity);

// Systems

fn update_animation(animations: Query<(&Animation, &AnimationType, &GlobalTransform)>) {
    for (animation, animation_type, transform) in animations.iter() {
        debug!("eased fraction: {}", animation.eased_fraction());
        debug!("animation translation: {}", transform.translation());
        debug!(
            "animation rotation: {}",
            transform.rotation().to_scaled_axis()
        );
        match animation_type {
            AnimationType::Wiggle => {
                debug!("i'm a wiggler");
            }
            AnimationType::Pulse => {
                debug!("i'm a pulser");
            }
            AnimationType::Popup => {
                debug!("i'm poppin up");
            }
            AnimationType::Burst => {
                debug!("i'm bursting");
            }
        }
    }
}

fn update_animation_timer(
    mut animations: Query<(Entity, &mut Animation)>,
    time: Res<Time>,
    mut _animation_finished: EventWriter<AnimationFinished>,
) {
    debug!("updating animation timer {}", NAME);
    for (_entity, mut animation) in animations.iter_mut() {
        animation.timer.tick(time.delta());

        if animation.timer.finished() {
            //animation_finished.write(AnimationFinished(entity));
        }
    }
}

fn cleanup_animations(mut commands: Commands, mut animations: EventReader<AnimationFinished>) {
    for animation in animations.read() {
        commands.entity(animation.0).remove::<Animation>();
    }
}
