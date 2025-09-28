pub mod prelude {
    pub use super::PlayerPlugin;
}

use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerSpeed(50.))
            .add_systems(Startup, spawn_player)
            .add_systems(Update, player_move);
    }
}

// Components
#[derive(Component)]
pub struct Player;

// Resources
#[derive(Resource, Deref)]
struct PlayerSpeed(f32);

// Systems
fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Transform::from_xyz(0., 30.0, -25.0),
        // add other components as needed, e.g., a mesh or collider
    ));
}

fn player_move(
    mut player: Single<&mut Transform, (With<Player>, Without<Camera3d>)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    speed: Res<PlayerSpeed>,
) {
    let mut delta = Vec3::ZERO;
    if input.pressed(KeyCode::KeyA) {
        delta.x -= 1.;
    }
    if input.pressed(KeyCode::KeyD) {
        delta.x += 1.;
    }
    if input.pressed(KeyCode::KeyW) {
        delta.z -= 1.;
    }
    if input.pressed(KeyCode::KeyS) {
        delta.z += 1.;
    }

    let forward = player.forward().as_vec3() * delta.z;
    let right = player.right().as_vec3() * delta.x;
    let mut to_move = forward + right;
    to_move.y = 0.;
    to_move = to_move.normalize_or_zero();
    player.translation += to_move * time.delta_secs() * **speed;
}
