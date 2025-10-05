pub mod prelude {
    pub use super::PlayerPlugin;
}

use EulerRot::YXZ;
use bevy::input::common_conditions::input_just_released;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy::window::WindowFocused;
use bevy::window::{PrimaryWindow, Window};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerSettings::default())
            .insert_resource(KeyBindings::default())
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
#[derive(Resource)]
struct PlayerSettings {
    pub sensitivity: f32,
    pub move_speed: f32,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            sensitivity: 100.0,
            move_speed: 50.0,
        }
    }
}

#[derive(Resource)]
pub struct KeyBindings {
    // Fly mode
    pub move_forward: KeyCode,
    pub move_backward: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub move_ascend: KeyCode,
    pub move_descend: KeyCode,
    // pub toggle_cursor: KeyCode,
    // pub sprint: KeyCode,
    // Normal mode
    // pub jump: KeyCode,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            move_forward: KeyCode::KeyW,
            move_backward: KeyCode::KeyS,
            move_left: KeyCode::KeyA,
            move_right: KeyCode::KeyD,
            move_ascend: KeyCode::Space,
            move_descend: KeyCode::ControlLeft,
            // toggle_cursor: KeyCode::Escape,
            // sprint: KeyCode::LShift,
            // jump: KeyCode::Space,
        }
    }
}

// Systems
fn spawn_player(mut commands: Commands) {
    let player = commands
        .spawn((
            Player,
            Transform::from_xyz(0.0, 5.0, 25.0),
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
    mut player: Single<&mut Transform, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    primary_window: Single<&Window, With<PrimaryWindow>>,
    player_settings: Res<PlayerSettings>,
    key_bindings: Res<KeyBindings>,
) {
    let mut velocity = Vec3::ZERO;

    if primary_window.cursor_options.grab_mode == CursorGrabMode::None {
        return;
    }

    if input.pressed(key_bindings.move_left) {
        velocity.x -= 1.;
    }
    if input.pressed(key_bindings.move_right) {
        velocity.x += 1.;
    }
    if input.pressed(key_bindings.move_forward) {
        velocity.z += 1.;
    }
    if input.pressed(key_bindings.move_backward) {
        velocity.z -= 1.;
    }

    if input.pressed(key_bindings.move_ascend) {
        velocity.y += 1.;
    }
    if input.pressed(key_bindings.move_descend) {
        velocity.y -= 1.;
    }

    // early return if no movement
    if velocity == Vec3::ZERO {
        return;
    }

    let forward = player.forward().as_vec3() * velocity.z;
    let right = player.right().as_vec3() * velocity.x;
    let up = player.up().as_vec3() * velocity.y;
    let direction = (forward + right + up).normalize();
    player.translation += direction * time.delta_secs() * player_settings.move_speed;
}

// Camera and player rotation system
fn player_look(
    mut player: Single<&mut Transform, (With<Player>, Without<Camera3d>)>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
    window: Single<&Window, With<PrimaryWindow>>,
    player_settings: Res<PlayerSettings>,
) {
    if !window.focused {
        return;
    }

    let dt = time.delta_secs();
    let adjusted_sens = player_settings.sensitivity / window.width().max(window.height());

    let (mut yaw, mut pitch, _roll) = player.rotation.to_euler(YXZ);
    pitch -= mouse_motion.delta.y * adjusted_sens * dt;
    yaw -= mouse_motion.delta.x * adjusted_sens * dt;
    pitch = pitch.clamp(-1.54, 1.54); // prevent gimbal lock (-pi / 2., pi / 2.)
    player.rotation = Quat::from_euler(YXZ, yaw, pitch, 0.);
}

// Grab cursor
#[derive(Event, Deref)]
struct GrabEvent(bool);

fn apply_grab(grab: Trigger<GrabEvent>, mut window: Single<&mut Window, With<PrimaryWindow>>) {
    if **grab {
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
        window.cursor_options.visible = false;
    } else {
        window.cursor_options.grab_mode = CursorGrabMode::None;
        window.cursor_options.visible = true;
    }
}

// When the window is focused, apply the grab state
fn focus_events(mut events: EventReader<WindowFocused>, mut commands: Commands) {
    if let Some(event) = events.read().last() {
        commands.trigger(GrabEvent(event.focused))
    }
}

// Can also toggle grab with Escape
// The function shouldnt toggle the window.focus state directly, that is bad code
fn toggle_grab(mut window: Single<&mut Window, With<PrimaryWindow>>, mut commands: Commands) {
    window.focused = !window.focused;
    commands.trigger(GrabEvent(window.focused))
}
