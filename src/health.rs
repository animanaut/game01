use bevy::app::Plugin;

use AppState::Running;
use bevy::prelude::*;

use crate::{app_states::AppState, tiles::TileCoordinate};

// Constants
const NAME: &str = "health";

// Plugin
pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app
            // events
            .add_event::<PickedUpHearts>()
            // systems
            .add_systems(OnEnter(Running), start_health)
            .add_systems(
                Update,
                (update_health, check_for_heart).run_if(in_state(Running)),
            )
            .add_systems(OnExit(Running), stop_health);
    }
}

// Components

/// health component
#[derive(Component, Debug)]
pub struct Health {
    pub hearts: Hearts,
    pub max: Hearts,
}

/// heart component to pickup
#[derive(Component, Debug, Copy, Clone)]
pub struct Hearts(pub usize);

// Resources

// Events
#[derive(Event)]
#[allow(dead_code)]
pub struct PickedUpHearts {
    pub entity: Entity,
    pub hearts: Hearts,
}

// Systems
fn start_health(mut _commands: Commands) {
    debug!("starting {}", NAME);
}

fn update_health() {
    debug!("updating {}", NAME);
}

fn check_for_heart(
    mut commands: Commands,
    mut health_bearer: Query<(&TileCoordinate, &mut Health, Entity)>,
    hearts: Query<(&TileCoordinate, &Hearts, Entity)>,
    mut event: EventWriter<PickedUpHearts>,
) {
    debug!("checking hearts {}", NAME);
    for (health_coordinate, mut health, has_health) in health_bearer.iter_mut() {
        for (heart_coordinate, hearts, heart) in hearts.iter() {
            if health_coordinate.eq2d(heart_coordinate)
                && health.hearts.0 + hearts.0 <= health.max.0
            {
                health.hearts.0 += hearts.0;
                event.write(PickedUpHearts {
                    entity: has_health,
                    hearts: *hearts,
                });
                commands.entity(heart).despawn();
            }
        }
    }
}

fn stop_health(mut _commands: Commands) {
    debug!("stopping {}", NAME);
}

// helper functions

// tests
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn should_pickup_heart() {
        // given
        let mut app = App::new();

        // when
        app.add_event::<PickedUpHearts>();
        app.add_systems(Update, check_for_heart);
        let entity = app
            .world_mut()
            .spawn(Health {
                hearts: Hearts(1),
                max: Hearts(2),
            })
            .insert(TileCoordinate::default())
            .id();
        let heart = app
            .world_mut()
            .spawn(Hearts(1))
            .insert(TileCoordinate::default())
            .id();
        app.update();

        // then
        assert!(app.world().get::<Hearts>(heart).is_none());
        assert_eq!(app.world().get::<Health>(entity).unwrap().hearts.0, 2);
    }

    #[test]
    fn should_not_pickup_heart_at_max_health() {
        // given
        let mut app = App::new();

        // when
        app.add_event::<PickedUpHearts>();
        app.add_systems(Update, check_for_heart);
        let entity = app
            .world_mut()
            .spawn(Health {
                hearts: Hearts(2),
                max: Hearts(2),
            })
            .insert(TileCoordinate::default())
            .id();
        let heart = app
            .world_mut()
            .spawn(Hearts(1))
            .insert(TileCoordinate::default())
            .id();
        app.update();

        // then
        assert!(app.world().get::<Hearts>(heart).is_some());
        assert_eq!(app.world().get::<Health>(entity).unwrap().hearts.0, 2);
    }
}
