//! Game settings can either be shown from the title screen (as the Settings Screen)
//! or overlaid on the game as a Settings Menu. In this example the UI is mostly the
//! same, but you could customise each menu.

use bevy::prelude::*;

use super::{
    pause::{IsPaused, PauseState},
    Screen,
};
use crate::theme::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<SettingsMenu>();

    app.add_systems(OnEnter(Screen::Settings), show_settings_screen);
    app.add_systems(OnEnter(PauseState::SettingsMenu), show_settings_menu);

    // The Settings Screen is despawned using [`StateScoped`], so no specific system is required.
    app.add_systems(OnExit(PauseState::SettingsMenu), despawn_settings_menu);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct SettingsMenu;

/// Spawns the actual settings UI.
fn spawn_settings_ui_elements(children: &mut ChildBuilder) {
    children.header("Settings");
    children.button("Back").observe(exit_settings);
}

/// Shows the Settings Menu, which is wrapped in a node with a background colour.
fn show_settings_menu(mut commands: Commands) {
    commands
        .menu_root()
        .insert(SettingsMenu)
        .with_children(spawn_settings_ui_elements);
}

/// Shows the Settings Screen, which is not wrapped in another menu node.
fn show_settings_screen(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Settings))
        .with_children(spawn_settings_ui_elements);
}

fn despawn_settings_menu(
    mut commands: Commands,
    settings_menu_query: Query<Entity, With<SettingsMenu>>,
) {
    for entity in &settings_menu_query {
        commands.entity(entity).despawn_recursive();
    }
}

fn exit_settings(
    _trigger: Trigger<OnPress>,
    is_paused: Option<Res<State<IsPaused>>>,
    next_pause_state: Option<ResMut<NextState<PauseState>>>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    match is_paused {
        Some(is_paused) => match is_paused.get() {
            IsPaused::Paused => {
                let Some(mut next_pause_state) = next_pause_state else {
                    return;
                };

                // If we are paused, then we should move back to the pause menu.
                next_pause_state.set(PauseState::PauseMenu);
            }
            _ => {}
        },
        None => {
            // If there is no [`IsPaused`], then we're accessing this from the menu.
            next_screen.set(Screen::Title);
        }
    }
}
