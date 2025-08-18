use AppState::MainMenu;
use bevy::prelude::*;

use crate::{app_states::AppState, gold::FinalPlayerGoldAmount};

// Constants
const NAME: &str = "main_menu";

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

// Plugin
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainMenu), start_main_menu)
            .add_systems(
                Update,
                (update_main_menu, report_last_run).run_if(in_state(MainMenu)),
            )
            .add_systems(OnExit(MainMenu), stop_main_menu);
    }
}

// Components
#[derive(Component, Debug)]
struct ButtonTargetState(AppState);

// Resources
#[derive(Resource)]
struct MainMenuData {
    main_menu_layout: Entity,
}

// Events

// Systems
fn start_main_menu(mut commands: Commands) {
    debug!("starting {}", NAME);

    // Layout
    // Top-level grid (app frame)
    let main_menu_layout = commands
        .spawn(Node {
            display: Display::Grid,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            grid_template_columns: vec![GridTrack::auto()],
            grid_template_rows: vec![GridTrack::auto(), GridTrack::flex(1.0), GridTrack::px(20.)],
            ..default()
        })
        .insert(Name::new("Main Layout"))
        .with_children(|builder| {
            // Header
            builder
                .spawn(Node {
                    display: Display::Grid,
                    justify_items: JustifyItems::Center,
                    padding: UiRect::all(Val::Px(12.0)),
                    ..default()
                })
                .insert(Name::new("Header"))
                .with_children(|builder| {
                    spawn_nested_text_bundle(builder, 40.0, "Main Menu");
                    spawn_nested_text_bundle(builder, 10.0, "");
                });
            // Main
            builder
                .spawn(Node {
                    display: Display::Grid,
                    justify_items: JustifyItems::Center,
                    padding: UiRect::all(Val::Px(12.0)),
                    grid_template_columns: RepeatedGridTrack::flex(2, 1.0),
                    ..default()
                })
                .insert(Name::new("Main"))
                .with_children(|builder| {
                    let start_name = Name::new("Start Game");
                    spawn_button_bundle(
                        builder,
                        start_name.clone(),
                        start_name.as_str(),
                        ButtonTargetState(AppState::Running),
                    );
                    let quit_name = Name::new("GGs");
                    spawn_button_bundle(
                        builder,
                        quit_name.clone(),
                        quit_name.as_str(),
                        ButtonTargetState(AppState::Quitting),
                    );
                });
        })
        .id();

    // insert resource
    commands.insert_resource(MainMenuData { main_menu_layout });
}

fn report_last_run(mut final_gold: EventReader<FinalPlayerGoldAmount>) {
    for final_player_gold in final_gold.read() {
        debug!(
            "final gold from last run: {}",
            final_player_gold.coins.coins
        );
    }
}

fn update_main_menu(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonTargetState),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    debug!("updating {}", NAME);
    for (interaction, mut color, target_state) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                debug!("button pressed");
                *color = PRESSED_BUTTON.into();
                next_state.set(target_state.0.clone());
            }
            Interaction::Hovered => {
                debug!("button hovered");
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                debug!("button normal");
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn stop_main_menu(mut commands: Commands, menu_data: Res<MainMenuData>) {
    debug!("stopping {}", NAME);
    commands.entity(menu_data.main_menu_layout).despawn();
}

// helper functions
fn spawn_nested_text_bundle(child_commands: &mut ChildSpawnerCommands, font_size: f32, text: &str) {
    child_commands
        .spawn(Text::new(text))
        .insert(TextFont {
            font_size,
            ..default()
        })
        .insert(TextColor(Color::srgb(0.9, 0.9, 0.9)));
}

fn spawn_button_bundle(
    child_commands: &mut ChildSpawnerCommands,
    button_name_component: Name,
    button_text: &str,
    button_target_state: ButtonTargetState,
) {
    child_commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
        .insert(button_name_component.clone())
        .with_children(|parent| {
            parent
                .spawn(Button)
                .insert(Node {
                    width: Val::Px(150.),
                    height: Val::Px(110.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .insert(button_name_component)
                .with_children(|parent| {
                    parent
                        .spawn(Text::new(button_text))
                        .insert(TextFont {
                            font_size: 40.0,
                            ..default()
                        })
                        .insert(TextColor(Color::srgb(0.9, 0.9, 0.9)));
                })
                .insert(button_target_state);
        });
}

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
