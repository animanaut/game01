use std::{fmt::Display, time::Duration};

use bevy::app::Plugin;
use log::debug;

use crate::{
    AppState::Running,
    controls::PlayerControlled,
    gold::Gold,
    tiles::{FloorTile, SolidTile, Tile},
    tutorial::Tutorial,
};
use bevy::prelude::*;

use crate::tiles::TileCoordinate;

// Constants
const NAME: &str = "sprites";

pub const SPRITE_DIM: u32 = 16;
pub const SPRITE_SCALE: f32 = 6.0;

const X_TILES: u32 = 49;
const Y_TILES: u32 = 22;
const GAP: u32 = 1;

const TILE_SHEET_FILE: &str = "Tilesheet/monochrome-transparent.png";
pub const ANIM_DURATION: u64 = 200;

// Enums
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
#[allow(dead_code)]
pub enum SpriteSheetTile {
    // creature sprites
    Player01,
    // exit tiles
    LevelExit01,
    // doors
    LockedDoor,
    MagicDoor,
    OpenDoor,
    RegularDoor,
    MechanicDoor,
    // floor tiles
    #[default]
    Grass,
    GrassFlowers,
    LongGrass,
    // walls
    BrickWall01,
    // valuables
    GoldCoin,
    GoldCoins,
    GoldCoinBag,
    // characters
    A,
    D,
    S,
    W,
    // controls
    LeftDigiPadRound,
    RightDigiPadRound,
    UpDigiPadRound,
    DownDigiPadRound,
}

impl SpriteSheetTile {
    fn index(&self) -> usize {
        match self {
            SpriteSheetTile::Player01 => SpriteSheetTile::get_index(30, 9),
            SpriteSheetTile::LevelExit01 => SpriteSheetTile::get_index(2, 9),
            SpriteSheetTile::LockedDoor => SpriteSheetTile::get_index(0, 9),
            SpriteSheetTile::MagicDoor => SpriteSheetTile::get_index(1, 9),
            SpriteSheetTile::OpenDoor => SpriteSheetTile::get_index(2, 9),
            SpriteSheetTile::RegularDoor => SpriteSheetTile::get_index(3, 9),
            SpriteSheetTile::MechanicDoor => SpriteSheetTile::get_index(4, 9),
            SpriteSheetTile::Grass => SpriteSheetTile::get_index(5, 0),
            SpriteSheetTile::GrassFlowers => SpriteSheetTile::get_index(6, 0),
            SpriteSheetTile::LongGrass => SpriteSheetTile::get_index(7, 0),
            SpriteSheetTile::BrickWall01 => SpriteSheetTile::get_index(10, 17),
            SpriteSheetTile::GoldCoin => SpriteSheetTile::get_index(41, 3),
            SpriteSheetTile::GoldCoins => SpriteSheetTile::get_index(41, 4),
            SpriteSheetTile::GoldCoinBag => SpriteSheetTile::get_index(42, 4),
            SpriteSheetTile::A => SpriteSheetTile::get_index(35, 18),
            SpriteSheetTile::D => SpriteSheetTile::get_index(38, 18),
            SpriteSheetTile::S => SpriteSheetTile::get_index(40, 19),
            SpriteSheetTile::W => SpriteSheetTile::get_index(44, 19),
            SpriteSheetTile::LeftDigiPadRound => SpriteSheetTile::get_index(47, 11),
            SpriteSheetTile::RightDigiPadRound => SpriteSheetTile::get_index(45, 11),
            SpriteSheetTile::UpDigiPadRound => SpriteSheetTile::get_index(44, 11),
            SpriteSheetTile::DownDigiPadRound => SpriteSheetTile::get_index(46, 11),
        }
    }

    fn get_index(x: usize, y: usize) -> usize {
        y * X_TILES as usize + x
    }

    fn color(&self) -> Color {
        match self {
            SpriteSheetTile::LevelExit01 => Color::linear_rgb(0.0, 1.0, 1.0),
            SpriteSheetTile::Grass => Color::linear_rgb(0.0, 1.0, 0.0),
            SpriteSheetTile::GrassFlowers => Color::linear_rgb(0.2, 1.0, 0.2),
            SpriteSheetTile::LongGrass => Color::linear_rgb(0.0, 1.0, 0.0),
            SpriteSheetTile::BrickWall01 => Color::linear_rgb(0.5, 0.1, 0.1),
            SpriteSheetTile::GoldCoin => Color::linear_rgb(0.6, 0.6, 0.0),
            SpriteSheetTile::GoldCoins => Color::linear_rgb(0.6, 0.6, 0.0),
            SpriteSheetTile::GoldCoinBag => Color::linear_rgb(0.6, 0.6, 0.0),
            _ => Color::default(),
        }
    }
}

// Plugin
pub struct SpritesPlugin;

impl Plugin for SpritesPlugin {
    fn build(&self, app: &mut App) {
        app
            // events
            .add_event::<MoveAnimationFinished>()
            .add_event::<SpawnSprite>()
            // systems
            .add_systems(OnEnter(Running), setup)
            .add_systems(
                Update,
                (update_animation_timer, cleanup_animations, spawn_sprite)
                    .run_if(in_state(Running)),
            )
            .add_systems(OnExit(Running), cleanup);
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

impl Default for MoveAnimation {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_millis(ANIM_DURATION), TimerMode::Once),
            function: EaseFunction::CircularInOut,
            start: Default::default(),
            end: Default::default(),
        }
    }
}

// Resources
#[derive(Resource, Clone)]
pub struct SpritesheetTexture(pub Handle<Image>);

#[derive(Resource, Clone)]
pub struct SpritesheetTextureAtlasLayout(pub Handle<TextureAtlasLayout>);

// Events
#[derive(Event)]
pub struct MoveAnimationFinished(Entity);

#[derive(Event, Default)]
pub struct SpawnSprite {
    pub coordinate: TileCoordinate,
    pub tile: SpriteSheetTile,
    /// custom color, will override defaults
    pub color: Option<Color>,
    pub tutorial: bool,
}

impl SpawnSprite {
    fn color(&self) -> Color {
        self.color.unwrap_or(self.tile.color())
    }
}

// Systems
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    debug!("starting {}", NAME);

    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(SPRITE_DIM),
        X_TILES,
        Y_TILES,
        Some(UVec2::splat(GAP)),
        None,
    );
    commands.insert_resource(SpritesheetTextureAtlasLayout(
        texture_atlas_layouts.add(layout),
    ));
    commands.insert_resource(SpritesheetTexture(asset_server.load(TILE_SHEET_FILE)));
}

fn cleanup(mut commands: Commands) {
    // TODO: do i need to cleanup the assets too?
    commands.remove_resource::<SpritesheetTexture>();
    commands.remove_resource::<SpritesheetTextureAtlasLayout>();
}

fn spawn_sprite(
    mut commands: Commands,
    mut spawn_coordinate: EventReader<SpawnSprite>,
    sprite_sheet: Res<SpritesheetTexture>,
    sprite_sheet_texture_atlas_layout: Res<SpritesheetTextureAtlasLayout>,
) {
    for spawn_sprite in spawn_coordinate.read() {
        debug!(
            "spawning sprite {:?} on coordinate: {}",
            spawn_sprite.tile, spawn_sprite.coordinate
        );

        // sprite
        let transform: Transform = spawn_sprite.coordinate.clone().into();
        let new_sprite = commands
            .spawn((
                MySprite,
                Sprite {
                    image: sprite_sheet.0.clone(),
                    color: spawn_sprite.color(),
                    texture_atlas: Some(TextureAtlas {
                        layout: sprite_sheet_texture_atlas_layout.0.clone(),
                        index: spawn_sprite.tile.index(),
                    }),
                    ..default()
                },
                transform,
                spawn_sprite.coordinate.clone(),
            ))
            .id();

        // custom marker components
        match spawn_sprite.tile {
            SpriteSheetTile::LevelExit01 => {
                commands.entity(new_sprite).insert(ExfilSprite);
                commands.entity(new_sprite).insert(Tile);
            }
            SpriteSheetTile::Player01 => {
                commands.entity(new_sprite).insert(PlayerControlled);
            }
            SpriteSheetTile::GoldCoin => {
                commands.entity(new_sprite).insert(Gold { coins: 1 });
            }
            SpriteSheetTile::GoldCoins => {
                commands.entity(new_sprite).insert(Gold { coins: 5 });
            }
            SpriteSheetTile::GoldCoinBag => {
                commands.entity(new_sprite).insert(Gold { coins: 25 });
            }
            SpriteSheetTile::BrickWall01 => {
                commands.entity(new_sprite).insert(SolidTile);
                commands.entity(new_sprite).insert(Tile);
            }
            SpriteSheetTile::Grass => {
                commands.entity(new_sprite).insert(Tile);
                commands.entity(new_sprite).insert(FloorTile);
            }
            SpriteSheetTile::GrassFlowers => {
                commands.entity(new_sprite).insert(Tile);
                commands.entity(new_sprite).insert(FloorTile);
            }
            SpriteSheetTile::LongGrass => {
                commands.entity(new_sprite).insert(Tile);
                commands.entity(new_sprite).insert(FloorTile);
            }
            _ => (),
        }

        if spawn_sprite.tutorial {
            commands.entity(new_sprite).insert(Tutorial);
        }
    }
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
