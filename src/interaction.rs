use bevy::app::Plugin;

use AppState::Running;
use bevy::prelude::*;

use crate::{app_states::AppState, movement::InteractionTriggered};

// Constants
const NAME: &str = "interaction";

// Plugin
pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app
            // events
            .add_event::<Interacted>()
            // systems
            .add_systems(OnEnter(Running), start_interaction)
            .add_systems(Update, (update_interaction).run_if(in_state(Running)))
            .add_systems(
                Update,
                (interaction_triggered)
                    .run_if(in_state(Running))
                    .run_if(on_event::<InteractionTriggered>),
            )
            .add_systems(OnExit(Running), stop_interaction);
    }
}

// Components
#[derive(Debug, Clone, PartialEq)]
pub struct InteractionId(pub u32);

#[derive(Component)]
pub struct InteractionSource(pub InteractionId);

#[derive(Component)]
pub struct InteractionTarget(pub InteractionId);

// Resources

// Events
#[derive(Event)]
pub struct Interacted(pub Entity);

// Systems
fn start_interaction(mut _commands: Commands) {
    debug!("starting {}", NAME);
}

fn update_interaction() {
    debug!("updating {}", NAME);
}

fn interaction_triggered(
    mut triggered: EventReader<InteractionTriggered>,
    sources: Query<(Entity, &InteractionSource)>,
    targets: Query<(Entity, &InteractionTarget)>,
    mut interacted: EventWriter<Interacted>,
) {
    debug!("interaction triggered {}", NAME);
    for t in triggered.read() {
        debug!("interaction triggered : event received");
        // TODO: maybe transfer all of this to a resource later
        if let Ok((_, source)) = sources.get(t.interacted_with) {
            debug!("interaction triggered : source found");
            for (target_entity, _) in targets.iter().filter(|(_, t)| source.0.eq(&t.0)) {
                debug!("interaction triggered : target found");
                interacted.write(Interacted(target_entity));
                debug!("interaction triggered : event sent");
            }
        }
    }
}

fn stop_interaction(mut _commands: Commands) {
    debug!("stopping {}", NAME);
}

// helper functions

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
