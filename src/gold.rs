use bevy::app::Plugin;

use AppState::Running;
use bevy::prelude::*;

use crate::{app_states::AppState, controls::PlayerControlled, tiles::TileCoordinate};

// Constants
const NAME: &str = "gold";

// Plugin
pub struct GoldPlugin;

impl Plugin for GoldPlugin {
    fn build(&self, app: &mut App) {
        app
            // events
            .add_event::<PlayerPickedUpGoldCoins>()
            // systems
            .add_systems(OnEnter(Running), start_gold)
            .add_systems(
                Update,
                (add_gold_to_player, log_player_gold, check_for_gold).run_if(in_state(Running)),
            )
            .add_systems(OnExit(Running), stop_gold);
    }
}

// Components
#[derive(Component, Clone, Copy)]
pub struct Gold {
    pub coins: i64,
}

// Resources

// Events
#[derive(Event)]
pub struct PlayerPickedUpGoldCoins {
    pub player: Entity,
    pub coins: Gold,
}

// Systems
fn start_gold() {
    debug!("start_gold {}", NAME);
}

fn add_gold_to_player(mut commands: Commands, mut players: Query<Entity, Added<PlayerControlled>>) {
    // TODO: transfer from resource to player on level change
    for player in players.iter_mut() {
        commands.entity(player).insert(Gold { coins: 0 });
        debug!("added 0 gold to player {}", player);
    }
}

fn check_for_gold(
    mut commands: Commands,
    mut players: Query<(&TileCoordinate, &mut Gold, Entity), With<PlayerControlled>>,
    coins: Query<(&TileCoordinate, &Gold, Entity), Without<PlayerControlled>>,
    mut event: EventWriter<PlayerPickedUpGoldCoins>,
) {
    debug!("checking gold {}", NAME);
    for (player_coordinate, mut player_coins, player) in players.iter_mut() {
        for (gold_coordinate, coins, gold) in coins.iter() {
            if player_coordinate.eq2d(gold_coordinate) {
                player_coins.coins += coins.coins;
                event.write(PlayerPickedUpGoldCoins {
                    player,
                    coins: *coins,
                });
                commands.entity(gold).despawn();
            }
        }
    }
}

fn log_player_gold(
    mut events: EventReader<PlayerPickedUpGoldCoins>,
    players: Query<&Gold, With<PlayerControlled>>,
) {
    for event in events.read() {
        if let Ok(player_gold) = players.get(event.player) {
            debug!(
                "player {} picked up {} gold. now has {} gold",
                event.player, event.coins.coins, player_gold.coins
            );
        }
    }
}

fn stop_gold(mut _commands: Commands) {
    debug!("stopping {}", NAME);
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
