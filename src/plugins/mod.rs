pub mod camera;
pub mod player;

// a prelude to simplify imports for consumers within the crate
pub mod prelude {
    pub use super::camera::CameraPlugin;
    pub use super::player::PlayerPlugin;
}

// aggregate app-level plugins here
use bevy::prelude::*;
use camera::CameraPlugin;
use player::PlayerPlugin;

pub struct PluginPack;

impl Plugin for PluginPack {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerPlugin, CameraPlugin));
    }
}
