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
/// Tile vs Sprites. Tiles have sprites as a representations, but a sprite can exist alone.
/// Tiles should be seen as level elements like walls/floors, while players enemies, valuables are
/// just sprites.
#[derive(Component)]
pub struct Tile;

#[derive(Component, PartialEq, Eq, Hash, Clone, Default, Debug)]
pub struct TileCoordinate {
    pub x: i32,
    pub y: i32,
    /// will be used for z depth
    pub z: i32,
}

impl TileCoordinate {
    pub fn eq2d(&self, other: &TileCoordinate) -> bool {
        self.x.eq(&other.x) && self.y.eq(&other.y)
    }
}

impl Display for TileCoordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TileCoordinate: x: {}, y: {}, z: {}",
            self.x, self.y, self.z
        )
    }
}

impl From<TileCoordinate> for Vec3 {
    fn from(val: TileCoordinate) -> Self {
        Vec3 {
            x: SPRITE_SCALE * SPRITE_DIM as f32 * val.x as f32,
            y: SPRITE_SCALE * SPRITE_DIM as f32 * val.y as f32,
            z: val.z as f32,
        }
    }
}

impl From<TileCoordinate> for Transform {
    fn from(val: TileCoordinate) -> Self {
        Transform::from_scale(Vec3::splat(SPRITE_SCALE)).with_translation(val.clone().into())
    }
}

/// marker for solid tiles like walls
#[derive(Component)]
pub struct SolidTile;

#[derive(Component)]
pub struct FloorTile;

#[derive(Component)]
#[allow(dead_code)]
pub struct InteractableTile;

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
        let tile = TileCoordinate { x: 2, y: 3, z: 0 };

        // when
        let default_vec3: Vec3 = Vec3::default();
        let vec3: Vec3 = tile.into();

        // then
        assert_eq!(default_vec3, default_tile.into());
        assert_eq!(
            Vec3::new(
                2.0 * SPRITE_DIM as f32 * SPRITE_SCALE,
                3.0 * SPRITE_DIM as f32 * SPRITE_SCALE,
                0.0
            ),
            vec3
        );
    }
}
