pub mod prelude {
    pub use super::CameraPlugin;
}

use crate::plugins::player::Player;
use bevy::input::common_conditions::input_just_released;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(
                Update,
                (
                    setup_camera,
                    player_look,
                    focus_events,
                    toggle_grab.run_if(input_just_released(KeyCode::Escape)),
                    log_camera_rotation,
                ),
            )
            .add_observer(apply_grab);
    }
}

// Components

// Resources

// Systems

// TODO: I dont like this shit, the fact that this thing runs every frame, just to return after
// running once
// I want to consider if its really worth it to make the camera a separate plugin. Or find a way to
// run the plugins one after another
fn setup_camera(
    mut commands: Commands,
    player_q: Query<Entity, With<Player>>,
    mut done: Local<bool>,
) {
    println!("Setting up camera");
    if *done {
        return;
    }
    if let Ok(player) = player_q.single() {
        let cam = commands
            .spawn((
                Camera3d::default(),
                Transform::from_xyz(0., 1.8, 0.), // eye height
                GlobalTransform::default(),
            ))
            .id();
        commands.entity(player).add_child(cam);
        println!("Camera entity: {:?}", cam);
        *done = true;
    }
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

fn log_camera_rotation(camera: Single<&Transform, With<Camera3d>>) {
    let (yaw, pitch, _roll) = camera.rotation.to_euler(EulerRot::YXZ);
    println!("Camera yaw: {yaw}, pitch: {pitch}");
    // println!("Camera position: {:?}", camera.translation);
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
