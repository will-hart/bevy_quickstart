//! The screen state for the main game loop.

use bevy::prelude::*;

use super::Screen;
use crate::{assets::BgmHandles, audio::bgm::BgmCommands as _, demo::level::SpawnLevel};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), spawn_level);
    app.add_systems(OnExit(Screen::Playing), stop_bgm);
}

fn spawn_level(mut commands: Commands) {
    commands.add(SpawnLevel);
    commands.play_bgm(BgmHandles::PATH_GAMEPLAY);
}

fn stop_bgm(mut commands: Commands) {
    commands.stop_bgm();
}
