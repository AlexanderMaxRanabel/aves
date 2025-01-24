use crate::components::{camera_sens_component::*, player_component::*, world_model_component::*};

use bevy::{
    color::palettes::tailwind, input::mouse::AccumulatedMouseMotion, prelude::*,
    render::view::RenderLayers, window::CursorGrabMode,
};

use std::f32::consts::FRAC_PI_2;

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(Vec2::new(0.003, 0.002))
    }
}
const DEFAULT_RENDER_LAYER: usize = 0;
const VIEW_MODEL_RENDER_LAYER: usize = 1;

pub fn spawn_view_model(mut commands: Commands) {
    /*let arm = meshes.add(Cuboid::new(0.01, 0.01, 0.4));
    let arm_material = materials.add(Color::from(tailwind::TEAL_200));*/

    commands
        .spawn((
            Player,
            CameraSensitivity::default(),
            Transform::from_xyz(0.0, 1.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            Visibility::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                WorldModelCamera,
                Camera3d::default(),
                Projection::from(PerspectiveProjection {
                    fov: 90.0_f32.to_radians(),
                    ..default()
                }),
            ));

            // Spawn view model camera.
            parent.spawn((
                Camera3d::default(),
                Camera {
                    // Bump the order to render on top of the world model.
                    order: 1,
                    ..default()
                },
                Projection::from(PerspectiveProjection {
                    fov: 70.0_f32.to_radians(),
                    ..default()
                }),
                // Only render objects belonging to the view model.
                RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
            ));
        });
}

pub fn spawn_world_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(50.0)));
    let cube = meshes.add(Cuboid::new(2.0, 0.5, 1.0));
    let torus = meshes.add(Torus::new(2.0, 0.5));
    let material = materials.add(Color::WHITE);
    commands.spawn((Mesh3d(floor), MeshMaterial3d(material.clone())));

    commands.spawn((
        Mesh3d(cube.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(0.0, 0.25, -3.0),
    ));

    commands.spawn((
        Mesh3d(cube.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(0.1, 0.45, -4.0),
    ));

    commands.spawn((
        Mesh3d(cube),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(0.75, 1.75, 0.0),
    ));

    commands.spawn((
        Mesh3d(torus.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(1.0, 0.4, 3.0),
    ));
    commands.spawn((
        Mesh3d(torus.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(2.0, 1.0, 5.0),
    ));
}

pub fn spawn_lights(mut commands: Commands) {
    commands.spawn((
        PointLight {
            color: Color::from(tailwind::ROSE_300),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-3.0, 4.0, -0.75),
        // The light source illuminates both the world model and the view model.
        RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
    ));
}

pub fn spawn_text(mut commands: Commands) {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        })
        .with_child(Text::new(concat!(
            "Move the camera with your mouse.\n",
            "Press arrow up to decrease the FOV of the world model.\n",
            "Press arrow down to increase the FOV of the world model.\n",
            "Press WASD to move around"
        )));
}