mod components;
mod spawners;

use components::{camera_sens_component::*, player_component::*, world_model_component::*};

use bevy::{
    color::palettes::tailwind, input::mouse::AccumulatedMouseMotion, prelude::*,
    render::view::RenderLayers, window::CursorGrabMode,
};
use std::f32::consts::FRAC_PI_2;

pub struct MainMethod;
impl Plugin for MainMethod {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                spawners::spawn_view_model,
                spawners::spawn_world_model,
                spawners::spawn_lights,
                spawners::spawn_text,
            ),
        );
        app.add_systems(Update, (move_camera, change_fov, player_movement));
        app.add_observer(spawn_cube);
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_plugins(MainMethod)
        .run();
}

fn move_camera(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    mut player: Query<(&mut Transform, &CameraSensitivity), With<Player>>,
) {
    let Ok((mut transform, camera_sensitivity)) = player.get_single_mut() else {
        return;
    };
    let delta = accumulated_mouse_motion.delta;

    if delta != Vec2::ZERO {
        let delta_yaw = -delta.x * camera_sensitivity.x;
        let delta_pitch = -delta.y * camera_sensitivity.y;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}

fn spawn_cube(
    _click: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut num: Local<usize>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.25 + 0.55 * *num as f32, 0.0),
    ));
    // With the MeshPickingPlugin added, you can add pointer event observers to meshes:
    *num += 1;
}

pub fn player_movement(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut window: Single<&mut Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut Transform, &Player)>,
) {
    if let Ok((mut transform, player)) = query.get_single_mut() {
        let mut speed: f32 = 0.5;
        let mut direction = Vec3::ZERO;

        // Forward/Backward movement
        if keyboard.pressed(KeyCode::KeyW) {
            direction.z -= 1.0;
            speed += 1.0;
        } else if keyboard.just_released(KeyCode::KeyW) {
            speed = 0.5;
        }

        if keyboard.pressed(KeyCode::KeyS) {
            direction.z += 1.0;
            speed += 1.0;
        } else if keyboard.just_released(KeyCode::KeyS) {
            speed = 0.5;
        }

        // Left/Right movement
        if keyboard.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
            speed += 1.0;
        } else if keyboard.just_released(KeyCode::KeyA) {
            speed = 0.5;
        }

        if keyboard.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
            speed += 1.0;
        } else if keyboard.just_released(KeyCode::KeyD) {
            speed = 0.5;
        }

        if keyboard.pressed(KeyCode::Space) {
            direction.y += 1.0;
            speed += 1.0;
        } else if keyboard.just_released(KeyCode::Space) {
            speed = 0.5;
        }

        if keyboard.pressed(KeyCode::ShiftLeft) {
            direction.y -= 1.0;
            speed += 1.0;
        } else if keyboard.just_released(KeyCode::ShiftLeft) {
            speed = 0.5;
        }

        if keyboard.pressed(KeyCode::KeyP) {
            std::process::exit(0);
        }

        if mouse.just_pressed(MouseButton::Middle) {
            window.cursor_options.visible = false;
            window.cursor_options.grab_mode = CursorGrabMode::Locked;
        }

        /*if keyboard.pressed(KeyCode::KeyG) {
            commands
                .spawn((
                    Text::new("Click Me!"),
                    Node {
                        position_type: PositionType::Absolute,
                        top: Val::Percent(12.0),
                        left: Val::Percent(12.0),
                        ..default()
                    },
                ))
                .observe(spawn_cube);
        }*/

        // Normalize the direction and apply movement
        if direction != Vec3::ZERO {
            direction = direction.normalize();
            transform.translation += direction * speed * time.delta_secs();
        }
    }
}

fn change_fov(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut world_model_projection: Query<&mut Projection, With<WorldModelCamera>>,
) {
    let Ok(mut projection) = world_model_projection.get_single_mut() else {
        return;
    };
    let Projection::Perspective(ref mut perspective) = projection.as_mut() else {
        unreachable!(
            "The `Projection` component was explicitly built with `Projection::Perspective`"
        );
    };

    if keyboard.pressed(KeyCode::ArrowUp) {
        perspective.fov -= 1.0_f32.to_radians();
        perspective.fov = perspective.fov.max(20.0_f32.to_radians());
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        perspective.fov += 1.0_f32.to_radians();
        perspective.fov = perspective.fov.min(160.0_f32.to_radians());
    }
}
