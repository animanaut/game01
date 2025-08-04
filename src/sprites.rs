use bevy::app::Plugin;

use AppState::Running;
use bevy::prelude::*;

use crate::app_states::AppState;

// Constants
const NAME: &str = "sprites";

const SPRITE_DIM: u32 = 16;
const X_TILES: u32 = 49;
const Y_TILES: u32 = 22;
const GAP: u32 = 1;

const HERO: usize = 9 as usize * X_TILES as usize + 30 as usize;

// Plugin
pub struct SpritesPlugin;

impl Plugin for SpritesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Running), start_sprite_atlas)
            .add_systems(Update, (update_sprite_atlas).run_if(in_state(Running)))
            .add_systems(OnExit(Running), stop_sprite_atlas);
    }
}

// Components
#[derive(Component)]
struct MySprite;

// Resources
#[derive(Resource, Clone)]
struct SpritesheetTexture(Handle<Image>);

// Events

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
        Sprite {
            image: sprite_sheet_texture.0.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: HERO,
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(6.0)).with_translation(Vec3::new(0.0, 0.0, 0.0)),
    ));
}

fn update_sprite_atlas() {
    debug!("updating {}", NAME);
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
