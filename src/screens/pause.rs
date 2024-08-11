//! Screens and states to handle pausing the game while in playing state.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use super::Screen;
use crate::theme::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_sub_state::<IsPaused>();

    app.register_type::<PauseMenu>();

    app.add_systems(
        Update,
        toggle_pause
            .run_if(in_state(Screen::Playing).and_then(input_just_pressed(KeyCode::Escape))),
    );

    app.add_systems(OnEnter(IsPaused::Paused), show_pause_menu);
    app.add_systems(OnExit(IsPaused::Paused), hide_pause_menu);
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
/// This marks [`IsPaused`] as a sub-state of the Playing screen.
#[source(Screen = Screen::Playing)]
pub enum IsPaused {
    #[default]
    Running,
    Paused,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct PauseMenu;

fn toggle_pause(current_state: Res<State<IsPaused>>, mut next_state: ResMut<NextState<IsPaused>>) {
    next_state.set(match current_state.get() {
        IsPaused::Running => IsPaused::Paused,
        IsPaused::Paused => IsPaused::Running,
    })
}

const PAUSE_MENU_BACKGROUND_COLOR: Color = Color::srgb(0.157, 0.157, 0.157);

fn show_pause_menu(mut commands: Commands) {
    commands
        .ui_root()
        .insert(PauseMenu)
        .with_children(|children| {
            children
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(500.),
                        height: Val::Px(150.),
                        padding: UiRect::all(Val::Px(10.)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(PAUSE_MENU_BACKGROUND_COLOR),
                    ..default()
                })
                .with_children(|children| {
                    children.header("Game Paused");
                    children.button("Menu").observe(return_to_title_screen);
                });
        });
}

fn hide_pause_menu(mut commands: Commands, pause_menu_query: Query<Entity, With<PauseMenu>>) {
    for menu in &pause_menu_query {
        commands.entity(menu).despawn_recursive();
    }
}

fn return_to_title_screen(_trigger: Trigger<OnPress>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
