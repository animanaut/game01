use bevy::app::Plugin;

use AppState::Running;
use bevy::prelude::*;

use crate::app_states::AppState;

// Constants
const NAME: &str = "tutorial";

// Plugin
pub struct TutorialPlugin;

impl Plugin for TutorialPlugin {
    fn build(&self, app: &mut App) {
        app
            // events
            .add_event::<CountDownTutorialCounter>()
            .add_event::<CountDownFinished>()
            // systems
            .add_systems(OnEnter(Running), start_tutorial)
            .add_systems(
                Update,
                (update_tutorial, countdown, countdown_finished).run_if(in_state(Running)),
            )
            .add_systems(OnExit(Running), stop_tutorial);
    }
}

// Components
/// marker component to mark tutorial entities
#[derive(Component)]
pub struct Tutorial;

/// counts down component
/// 0 is default, no countdown or countdown done if initial value was > 1
#[derive(Component, Default)]
pub struct TutorialCountdown(u64);

// Resources

// Events
#[derive(Event)]
pub struct CountDownTutorialCounter(Entity);

#[derive(Event)]
pub struct CountDownFinished(Entity);

// Systems
fn start_tutorial(mut _commands: Commands) {
    debug!("starting {}", NAME);
}

fn update_tutorial() {
    debug!("updating {}", NAME);
}

fn countdown(
    mut countdowns: EventReader<CountDownTutorialCounter>,
    mut finished: EventWriter<CountDownFinished>,
    mut counters: Query<(Entity, &mut TutorialCountdown)>,
) {
    for countdown in countdowns.read() {
        if let Ok((entity, mut counter)) = counters.get_mut(countdown.0) {
            if counter.0 != 0 {
                counter.0 -= 1;
                if counter.0 == 0 {
                    finished.write(CountDownFinished(entity));
                }
            }
        }
    }
}

fn countdown_finished(mut commands: Commands, mut finished: EventReader<CountDownFinished>) {
    for f in finished.read() {
        commands.entity(f.0).remove::<TutorialCountdown>();
    }
}

fn stop_tutorial(mut _commands: Commands) {
    debug!("stopping {}", NAME);
}

// helper functions

// tests
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use std::borrow::BorrowMut;

    #[test]
    fn should_countdown_by_one() {
        // given
        let mut app = App::new();

        // when
        app.add_event::<CountDownTutorialCounter>();
        app.add_event::<CountDownFinished>();
        app.add_systems(Update, countdown);
        let entity = app
            .borrow_mut()
            .world_mut()
            .spawn(TutorialCountdown(2))
            .id();
        app.borrow_mut()
            .world_mut()
            .resource_mut::<Events<CountDownTutorialCounter>>()
            .send(CountDownTutorialCounter(entity));
        app.update();

        // then
        assert!(app.world().get::<TutorialCountdown>(entity).is_some());
        assert_eq!(app.world().get::<TutorialCountdown>(entity).unwrap().0, 1);
    }

    #[test]
    fn should_send_out_finished_event_after_finished() {
        // given
        let mut app = App::new();

        // when
        app.add_event::<CountDownTutorialCounter>();
        app.add_event::<CountDownFinished>();
        app.add_systems(Update, countdown);
        let entity = app
            .borrow_mut()
            .world_mut()
            .spawn(TutorialCountdown(1))
            .id();
        app.borrow_mut()
            .world_mut()
            .resource_mut::<Events<CountDownTutorialCounter>>()
            .send(CountDownTutorialCounter(entity));
        app.update();

        // then
        assert!(app.world().get::<TutorialCountdown>(entity).is_some());
        assert_eq!(app.world().get::<TutorialCountdown>(entity).unwrap().0, 0);

        let finished_events = app.world().resource::<Events<CountDownFinished>>();
        let mut finished_events_reader = finished_events.get_cursor();
        let actual_finished_event = finished_events_reader.read(finished_events).next();
        let expected_finished_event = CountDownFinished(entity);
        assert!(
            actual_finished_event.is_some(),
            "event CountdownFinished is present"
        );
        assert_eq!(
            expected_finished_event.0,
            actual_finished_event.unwrap().0,
            "CountdownFinished contains correct entity"
        );
    }

    #[test]
    fn should_remove_counter_after_finished() {
        // given
        let mut app = App::new();

        // when
        app.add_event::<CountDownFinished>();
        app.add_systems(Update, countdown_finished);
        let entity = app
            .borrow_mut()
            .world_mut()
            .spawn(TutorialCountdown(0))
            .id();
        app.borrow_mut()
            .world_mut()
            .resource_mut::<Events<CountDownFinished>>()
            .send(CountDownFinished(entity));
        app.update();

        // then
        assert!(app.world().get::<TutorialCountdown>(entity).is_none());
    }
}
