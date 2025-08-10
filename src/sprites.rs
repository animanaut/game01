use std::fmt::Display;

use bevy::app::Plugin;

use AppState::Running;
use bevy::prelude::*;

use crate::{app_states::AppState, controls::PlayerControlled, tiles::TileCoordinate};

// Constants
const NAME: &str = "sprites";

pub const SPRITE_DIM: u32 = 16;
pub const SPRITE_SCALE: f32 = 6.0;

const X_TILES: u32 = 49;
const Y_TILES: u32 = 22;
const GAP: u32 = 1;

const HERO: usize = 9_usize * X_TILES as usize + 30_usize;
const OPEN_DOOR_1: usize = 9_usize * X_TILES as usize + 2_usize;

// Plugin
pub struct SpritesPlugin;

impl Plugin for SpritesPlugin {
    fn build(&self, app: &mut App) {
        app
            // events
            .add_event::<MoveAnimationFinished>()
            // systems
            .add_systems(OnEnter(Running), start_sprite_atlas)
            .add_systems(
                Update,
                (
                    update_sprite_atlas,
                    update_animation_timer,
                    cleanup_animations,
                )
                    .run_if(in_state(Running)),
            )
            .add_systems(OnExit(Running), stop_sprite_atlas);
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
struct SpritesheetTexture(Handle<Image>);

// Events
#[derive(Event)]
pub struct MoveAnimationFinished(Entity);

// Systems
fn start_sprite_atlas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    debug!("starting {}", NAME);
    let sprite_sheet_texture =
        SpritesheetTexture(asset_server.load("Tilesheet/monochrome-transparent.png"));

    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(SPRITE_DIM),
        X_TILES,
        Y_TILES,
        Some(UVec2::splat(GAP)),
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands.insert_resource(sprite_sheet_texture.clone());

    commands.spawn((
        MySprite,
        PlayerControlled,
        Sprite {
            image: sprite_sheet_texture.0.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: HERO,
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(SPRITE_SCALE)).with_translation(Vec3::new(0.0, 0.0, 0.0)),
        TileCoordinate { x: 0, y: 0 },
    ));

    commands.spawn((
        MySprite,
        ExfilSprite,
        Sprite {
            image: sprite_sheet_texture.0.clone(),
            color: Color::linear_rgb(0.0, 1.0, 1.0),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: OPEN_DOOR_1,
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(SPRITE_SCALE)).with_translation(Vec3::new(
            SPRITE_SCALE * 2.0 * SPRITE_DIM as f32,
            0.0,
            0.0,
        )),
        TileCoordinate { x: 2, y: 0 },
    ));
}

fn update_sprite_atlas() {
    debug!("updating {}", NAME);
}

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

fn stop_sprite_atlas(mut commands: Commands, sprites: Query<Entity, With<MySprite>>) {
    debug!("stopping {}", NAME);
    commands.remove_resource::<SpritesheetTexture>();
    for sprite in sprites.iter() {
        commands.entity(sprite).despawn();
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
