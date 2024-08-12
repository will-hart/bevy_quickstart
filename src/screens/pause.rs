//! Screens and states to handle pausing the game while in playing state.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use ui_palette::MENU_BACKGROUND_COLOR;

use super::Screen;
use crate::theme::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_sub_state::<IsPaused>();
    app.add_sub_state::<PauseState>();

    app.register_type::<PauseMenu>();

    app.add_systems(
        Update,
        toggle_pause
            .run_if(in_state(Screen::Playing).and_then(input_just_pressed(KeyCode::Escape))),
    );

    app.add_systems(OnEnter(PauseState::PauseMenu), spawn_pause_menu);
    app.add_systems(OnExit(PauseState::PauseMenu), despawn_pause_menus);
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
/// This marks [`IsPaused`] as a sub-state of the Playing screen.
#[source(Screen = Screen::Playing)]
pub enum IsPaused {
    #[default]
    Running,
    Paused,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
/// This is used to determine what state the pause menu is in.
#[source(IsPaused = IsPaused::Paused)]
pub enum PauseState {
    #[default]
    PauseMenu,
    SettingsMenu,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct PauseMenu;

fn toggle_pause(
    current_pause_state: Option<Res<State<PauseState>>>,
    mut next_is_paused: ResMut<NextState<IsPaused>>,
    mut next_state: ResMut<NextState<PauseState>>,
) {
    match current_pause_state {
        Some(pause_state) => match pause_state.get() {
            PauseState::PauseMenu => next_is_paused.set(IsPaused::Running),
            PauseState::SettingsMenu => next_state.set(PauseState::PauseMenu),
        },
        None => {
            // We aren't currently paused as the resource doesn't exist.
            next_is_paused.set(IsPaused::Paused);
        }
    }
}

fn spawn_pause_menu(mut commands: Commands) {
    commands
        .ui_root()
        .insert(PauseMenu)
        .with_children(|children| {
            children
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(500.),
                        height: Val::Px(300.),
                        padding: UiRect::all(Val::Px(10.)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(MENU_BACKGROUND_COLOR),
                    ..default()
                })
                .with_children(|children| {
                    children.header("Game Paused");
                    children.button("Resume").observe(trigger_unpause_game);
                    children.button("Settings").observe(trigger_settings_menu);
                    children.button("Exit").observe(trigger_return_to_title);
                });
        });
}

fn despawn_pause_menus(mut commands: Commands, pause_menu_query: Query<Entity, With<PauseMenu>>) {
    for menu in &pause_menu_query {
        commands.entity(menu).despawn_recursive();
    }
}

fn trigger_return_to_title(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

fn trigger_unpause_game(
    _trigger: Trigger<OnPress>,
    mut next_is_paused: ResMut<NextState<IsPaused>>,
) {
    next_is_paused.set(IsPaused::Running);
}

fn trigger_settings_menu(
    _trigger: Trigger<OnPress>,
    mut next_state: ResMut<NextState<PauseState>>,
) {
    next_state.set(PauseState::SettingsMenu);
}
