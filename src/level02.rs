use bevy::app::Plugin;

use AppState::{MainMenu, Running};
use LevelState::Level02;
use bevy::prelude::*;

use crate::{
    app_states::{AppState, LevelState},
    controls::PlayerControlled,
    sprites::{
        ExfilSprite, GAP, HERO, MySprite, OPEN_DOOR_1, SPRITE_DIM, SPRITE_SCALE,
        SpritesheetTexture, X_TILES, Y_TILES,
    },
    tiles::TileCoordinate,
};

// Constants
const NAME: &str = "level02";

// Plugin
pub struct Level02Plugin;

impl Plugin for Level02Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Level02), start_level02)
            .add_systems(
                Update,
                (update_level01, check_for_exit_level02)
                    .run_if(in_state(Running))
                    .run_if(in_state(Level02)),
            )
            .add_systems(OnExit(Level02), stop_level02);
    }
}

// Components

// Resources

// Events

// Systems
fn start_level02(
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

fn update_level01() {
    debug!("updating {}", NAME);
}

fn check_for_exit_level02(
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

fn stop_level02(mut commands: Commands, sprites: Query<Entity, With<MySprite>>) {
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
