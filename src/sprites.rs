use std::fmt::Display;

use bevy::app::Plugin;

use crate::AppState::Running;
use bevy::prelude::*;

use crate::tiles::TileCoordinate;

// Constants
const NAME: &str = "sprites";

pub const SPRITE_DIM: u32 = 16;
pub const SPRITE_SCALE: f32 = 6.0;

pub const X_TILES: u32 = 49;
pub const Y_TILES: u32 = 22;
pub const GAP: u32 = 1;

pub const HERO: usize = 9_usize * X_TILES as usize + 30_usize;
pub const OPEN_DOOR_1: usize = 9_usize * X_TILES as usize + 2_usize;

// Plugin
pub struct SpritesPlugin;

impl Plugin for SpritesPlugin {
    fn build(&self, app: &mut App) {
        app
            // events
            .add_event::<MoveAnimationFinished>()
            // systems
            .add_systems(
                Update,
                (update_animation_timer, cleanup_animations).run_if(in_state(Running)),
            );
    }
}

// Components
#[derive(Component)]
pub struct MySprite;

#[derive(Component)]
pub struct ExfilSprite;

#[derive(Component)]
pub struct MoveAnimation {
    pub timer: Timer,
    pub function: EaseFunction,
    pub start: TileCoordinate,
    pub end: TileCoordinate,
}

impl Display for MoveAnimation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Animation: start: {}, end: {}, timer fraction: {}, timer remaining: {}",
            self.start,
            self.end,
            self.timer.fraction(),
            self.timer.remaining_secs()
        )
    }
}

// Resources
#[derive(Resource, Clone)]
pub struct SpritesheetTexture(pub Handle<Image>);

// Events
#[derive(Event)]
pub struct MoveAnimationFinished(Entity);

// Systems
fn update_animation_timer(
    mut animations: Query<(Entity, &mut MoveAnimation)>,
    time: Res<Time>,
    mut animation_finished: EventWriter<MoveAnimationFinished>,
) {
    debug!("updating move animation {}", NAME);
    for (entity, mut animation) in animations.iter_mut() {
        animation.timer.tick(time.delta());

        if animation.timer.finished() {
            animation_finished.write(MoveAnimationFinished(entity));
        }
    }
}

fn cleanup_animations(
    mut commands: Commands,
    mut move_animations: EventReader<MoveAnimationFinished>,
) {
    for move_animation in move_animations.read() {
        commands.entity(move_animation.0).remove::<MoveAnimation>();
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
