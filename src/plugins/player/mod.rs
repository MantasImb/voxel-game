pub mod prelude {
    pub use super::PlayerPlugin;
}

use bevy::input::common_conditions::input_just_released;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerSpeed(50.))
            .add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                (
                    player_move,
                    player_look,
                    focus_events,
                    toggle_grab.run_if(input_just_released(KeyCode::Escape)),
                ),
            )
            .add_observer(apply_grab);
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
    let player = commands
        .spawn((
            Player,
            Transform::from_xyz(0., 30.0, -25.0),
            // add other components as needed, e.g., a mesh or collider
        ))
        .id();
    let cam = commands
        .spawn((
            Camera3d::default(),
            Transform::from_xyz(0., 1.8, 0.), // eye height
            GlobalTransform::default(),
        ))
        .id();
    commands.entity(player).add_child(cam);
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
        delta.z += 1.;
    }
    if input.pressed(KeyCode::KeyS) {
        delta.z -= 1.;
    }

    let forward = player.forward().as_vec3() * delta.z;
    let right = player.right().as_vec3() * delta.x;
    let mut to_move = forward + right;
    to_move.y = 0.;
    to_move = to_move.normalize_or_zero();
    player.translation += to_move * time.delta_secs() * **speed;
}

// Camera and plauer rotation system
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::window::{PrimaryWindow, Window};
fn player_look(
    mut player: Single<&mut Transform, (With<Player>, Without<Camera3d>)>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    if !window.focused {
        return;
    }

    let dt = time.delta_secs();
    let sensitivity = 100. / window.width().max(window.height());

    use EulerRot::YXZ;
    let (mut yaw, mut pitch, _roll) = player.rotation.to_euler(YXZ);
    pitch -= mouse_motion.delta.y * sensitivity * dt;
    yaw -= mouse_motion.delta.x * sensitivity * dt;
    pitch = pitch.clamp(-1.54, 1.54); // prevent gimbal lock (-pi / 2., pi / 2.)
    player.rotation = Quat::from_euler(YXZ, yaw, pitch, 0.);
}

// Grab cursor
#[derive(Event, Deref)]
struct GrabEvent(bool);

fn apply_grab(grab: Trigger<GrabEvent>, mut window: Single<&mut Window, With<PrimaryWindow>>) {
    use bevy::window::CursorGrabMode;
    if **grab {
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
        window.cursor_options.visible = false;
    } else {
        window.cursor_options.grab_mode = CursorGrabMode::None;
        window.cursor_options.visible = true;
    }
}

// When the window is focused, apply the grab state
use bevy::window::WindowFocused;
fn focus_events(mut events: EventReader<WindowFocused>, mut commands: Commands) {
    if let Some(event) = events.read().last() {
        commands.trigger(GrabEvent(event.focused))
    }
}

// Can also toggle grab with Escape
fn toggle_grab(mut window: Single<&mut Window, With<PrimaryWindow>>, mut commands: Commands) {
    window.focused = !window.focused;
    commands.trigger(GrabEvent(window.focused))
}
