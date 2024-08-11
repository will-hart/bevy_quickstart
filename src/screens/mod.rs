//! The game's main screen states and transitions between them.

mod credits;
mod loading;
mod pause;
mod playing;
mod splash;
mod title;

pub use pause::IsPaused;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.enable_state_scoped_entities::<Screen>();

    app.add_plugins((
        splash::plugin,
        loading::plugin,
        pause::plugin,
        title::plugin,
        credits::plugin,
        playing::plugin,
    ));
}

/// The game's main screen states.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum Screen {
    #[default]
    Splash,
    Loading,
    Title,
    Credits,
    Playing,
}
