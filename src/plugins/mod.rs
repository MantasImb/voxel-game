pub mod player;

// a prelude to simplify imports for consumers within the crate
pub mod prelude {
    pub use super::player::PlayerPlugin;
}

// aggregate app-level plugins here
use bevy::prelude::*;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use player::PlayerPlugin;

pub struct PluginPack;

impl Plugin for PluginPack {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ScreenDiagnosticsPlugin::default(),
            ScreenFrameDiagnosticsPlugin,
        ))
        .add_plugins(PlayerPlugin);
    }
}
