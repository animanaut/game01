use std::{fmt::Display, time::Duration};

use bevy::app::Plugin;
use log::debug;

use crate::{AppState::Running, controls::PlayerControlled, gold::Gold, tutorial::Tutorial};
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
pub enum Tile {
    // creature sprites
    Player01,
    // exit tiles
    LevelExit01,
    // floor tiles
    #[default]
    Grass,
    GrassFlowers,
    LongGrass,
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

impl Tile {
    fn index(&self) -> usize {
        match self {
            Tile::Player01 => Tile::get_index(30, 9),
            Tile::LevelExit01 => Tile::get_index(2, 9),
            Tile::Grass => Tile::get_index(5, 0),
            Tile::GrassFlowers => Tile::get_index(6, 0),
            Tile::LongGrass => Tile::get_index(7, 0),
            Tile::GoldCoin => Tile::get_index(41, 3),
            Tile::GoldCoins => Tile::get_index(41, 4),
            Tile::GoldCoinBag => Tile::get_index(42, 4),
            Tile::A => Tile::get_index(35, 18),
            Tile::D => Tile::get_index(38, 18),
            Tile::S => Tile::get_index(39, 19),
            Tile::W => Tile::get_index(44, 19),
            Tile::LeftDigiPadRound => Tile::get_index(47, 11),
            Tile::RightDigiPadRound => Tile::get_index(45, 11),
            Tile::UpDigiPadRound => Tile::get_index(44, 11),
            Tile::DownDigiPadRound => Tile::get_index(46, 11),
        }
    }

    fn get_index(x: usize, y: usize) -> usize {
        y * X_TILES as usize + x
    }

    fn color(&self) -> Color {
        match self {
            Tile::LevelExit01 => Color::linear_rgb(0.0, 1.0, 1.0),
            Tile::Grass => Color::linear_rgb(0.0, 1.0, 0.0),
            Tile::GrassFlowers => Color::linear_rgb(0.2, 1.0, 0.2),
            Tile::LongGrass => Color::linear_rgb(0.0, 8.0, 0.0),
            Tile::GoldCoin => Color::linear_rgb(0.6, 0.6, 0.0),
            Tile::GoldCoins => Color::linear_rgb(0.6, 0.6, 0.0),
            Tile::GoldCoinBag => Color::linear_rgb(0.6, 0.6, 0.0),
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
    pub tile: Tile,
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
            Tile::LevelExit01 => {
                commands.entity(new_sprite).insert(ExfilSprite);
            }
            Tile::Player01 => {
                commands.entity(new_sprite).insert(PlayerControlled);
            }
            Tile::GoldCoin => {
                commands.entity(new_sprite).insert(Gold { coins: 1 });
            }
            Tile::GoldCoins => {
                commands.entity(new_sprite).insert(Gold { coins: 5 });
            }
            Tile::GoldCoinBag => {
                commands.entity(new_sprite).insert(Gold { coins: 25 });
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
