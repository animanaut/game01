use std::fmt::Display;

use bevy::app::Plugin;

use AppState::Running;
use bevy::prelude::*;

use crate::{
    app_states::AppState,
    sprites::{SPRITE_DIM, SPRITE_SCALE},
};

// Constants
const NAME: &str = "tiles";

// Plugin
pub struct TilesPlugin;

impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Running), start_tiles)
            .add_systems(Update, (logging_tiles).run_if(in_state(Running)))
            .add_systems(OnExit(Running), stop_tiles);
    }
}

// Components
#[derive(Component)]
#[allow(dead_code)]
pub struct Tile;

#[derive(Component, PartialEq, Clone, Default)]
pub struct TileCoordinate {
    pub x: i64,
    pub y: i64,
}

impl Display for TileCoordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TileCoordinate: x: {}, y: {}", self.x, self.y)
    }
}

impl Into<Vec3> for TileCoordinate {
    fn into(self) -> Vec3 {
        Vec3 {
            x: SPRITE_SCALE as f32 * SPRITE_DIM as f32 * self.x as f32,
            y: SPRITE_SCALE as f32 * SPRITE_DIM as f32 * self.y as f32,
            z: 0 as f32,
        }
    }
}

impl Into<Transform> for TileCoordinate {
    fn into(self) -> Transform {
        Transform::from_scale(Vec3::splat(SPRITE_SCALE)).with_translation(self.clone().into())
    }
}

// Resources

// Events

// Systems
fn start_tiles(mut _commands: Commands) {
    debug!("starting {}", NAME);
}

fn logging_tiles(tile_coordinates: Query<(Entity, &TileCoordinate)>) {
    debug!("logging {}", NAME);
    for (entity, coordinate) in tile_coordinates.iter() {
        debug!("entity: {}, coordinate: {}", entity, coordinate);
    }
}

fn stop_tiles(mut _commands: Commands) {
    debug!("stopping {}", NAME);
}

// helper functions

// tests
#[cfg(test)]
mod tests {
    use crate::sprites::SPRITE_SCALE;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn should_convert_tile_coordinate_into_vec3() {
        // given
        let default_tile = TileCoordinate::default();
        let tile = TileCoordinate { x: 2, y: 3 };

        // when
        let default_vec3: Vec3 = Vec3::default();
        let vec3: Vec3 = tile.into();

        // then
        assert_eq!(default_vec3, default_tile.into());
        assert_eq!(
            Vec3::new(
                2.0 * SPRITE_DIM as f32 * SPRITE_SCALE as f32,
                3.0 * SPRITE_DIM as f32 * SPRITE_SCALE as f32,
                0.0
            ),
            vec3
        );
    }
}
