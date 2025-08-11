use bevy::app::Plugin;

use AppState::Running;
use LevelState::Level01;
use LevelState::Level02;
use bevy::prelude::*;

use crate::sprites::SpawnPlayer;
use crate::{
    app_states::{AppState, LevelState},
    controls::PlayerControlled,
    sprites::{
        ExfilSprite, GAP, MySprite, OPEN_DOOR_1, SPRITE_DIM, SPRITE_SCALE, SpritesheetTexture,
        X_TILES, Y_TILES,
    },
    tiles::TileCoordinate,
};

// Constants
const NAME: &str = "level01";

// Plugin
pub struct Level01Plugin;

impl Plugin for Level01Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Level01), start_level01)
            .add_systems(
                Update,
                (update_level01, check_for_exit_level01)
                    .run_if(in_state(Running))
                    .run_if(in_state(Level01)),
            )
            .add_systems(OnExit(Level01), stop_level01);
    }
}

// Components

// Resources

// Events

// Systems
fn start_level01(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut spawn_player: EventWriter<SpawnPlayer>,
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

    spawn_player.write(SpawnPlayer(TileCoordinate { x: 0, y: 0 }));

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

fn check_for_exit_level01(
    mut next_state: ResMut<NextState<LevelState>>,
    players: Query<&TileCoordinate, (With<PlayerControlled>, Without<ExfilSprite>)>,
    exfils: Query<&TileCoordinate, (With<ExfilSprite>, Without<PlayerControlled>)>,
) {
    debug!("checking exit {}", NAME);
    if let Ok(player_coordinate) = players.single() {
        for exfil_coordinate in exfils.iter() {
            if player_coordinate.eq(exfil_coordinate) {
                // TODO: smoother transition, maybe with animation on an event
                debug!("changing LevelState to {:?}", Level02);
                next_state.set(Level02);
            }
        }
    }
}

fn stop_level01(mut commands: Commands, sprites: Query<Entity, With<MySprite>>) {
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
