use bevy::app::Plugin;

use AppState::Running;
use bevy::prelude::*;

use crate::{app_states::AppState, controls::PlayerControlled, health::Health};

// Constants
const NAME: &str = "in_game_ui";

// Plugin
pub struct InGameUIPlugin;

impl Plugin for InGameUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Running), start_ingame_ui)
            .add_systems(Update, (added_health_to_player).run_if(in_state(Running)))
            .add_systems(
                Update,
                (changed_health_of_player)
                    .run_if(resource_exists::<HeartUIRoot>)
                    .run_if(in_state(Running)),
            )
            .add_systems(OnExit(Running), stop_ingame_ui);
    }
}

// Components
#[derive(Component)]
struct InGameUI;

// Resources
#[derive(Resource, Clone, Copy)]
struct HeartUIRoot(Entity);

// Events

// Systems
fn start_ingame_ui() {}

fn added_health_to_player(
    mut commands: Commands,
    player_health_added: Query<(Entity, &Health), (With<PlayerControlled>, Added<Health>)>,
    mut health_ui_res: Option<ResMut<HeartUIRoot>>,
) {
    for (_player, health) in player_health_added.iter() {
        debug!("player health added ui {}", NAME);
        let hearts_root = commands.spawn(heart_ui_root(health)).id();
        if let Some(res) = &mut health_ui_res {
            commands.entity(res.0).despawn();
            commands.remove_resource::<HeartUIRoot>();
            commands.insert_resource(HeartUIRoot(hearts_root));
        } else {
            commands.insert_resource(HeartUIRoot(hearts_root));
        }
        debug!("player health added ui {}", hearts_root);
    }
}

fn changed_health_of_player(
    mut commands: Commands,
    player_health_changed: Query<(Entity, &Health), (With<PlayerControlled>, Changed<Health>)>,
    health_ui_res: Res<HeartUIRoot>,
) {
    for (_player, health) in player_health_changed.iter() {
        let hearts_root = commands.spawn(heart_ui_root(health)).id();
        commands.entity(health_ui_res.0).despawn();
        commands.remove_resource::<HeartUIRoot>();
        commands.insert_resource(HeartUIRoot(hearts_root));
        debug!("player health changed ui {}", hearts_root);
    }
}

fn stop_ingame_ui(mut commands: Commands, ui: Query<Entity, With<InGameUI>>) {
    debug!("stopping {}", NAME);
    for x in ui.iter() {
        commands.entity(x).despawn();
    }
    commands.remove_resource::<HeartUIRoot>();
}

// helper functions
fn heart_ui_root(health: &Health) -> impl Bundle + use<> {
    (
        InGameUI,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(Text::new(format!(
            "Health: {:?}/{:?}",
            health.hearts.0, health.max.0
        )),)],
    )
}

// tests
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    //use super::*;
    //use std::borrow::BorrowMut;

    /*
    #[test]
    fn should_test_something() {
        // given
        //let mut app = App::new();

        // when
        //app.add_event::<HealthDamageReceived>();
        //app.add_systems(Update, damage_received_listener);
        //let entity = app.borrow_mut().world().spawn(Health(100)).id();
        //app.borrow_mut().world().resource_mut::<Events<HealthDamageReceived>>().send(HealthDamageReceived { entity, damage: 10 });
        //app.update();

        // then
        //assert!(app.world().get::<Health>(entity).is_some());
        //assert_eq!(app.world().get::<Health>(entity).unwrap().0, 90);
    }
    */
}
