use crate::components::{camera_sens_component::*, player_component::*, world_model_component::*};
use rand::Rng;

use bevy::{
    color::palettes::tailwind, pbr::CLUSTERED_FORWARD_STORAGE_BUFFER_COUNT, prelude::*,
    render::view::RenderLayers,
};

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(Vec2::new(0.003, 0.002))
    }
}
const DEFAULT_RENDER_LAYER: usize = 0;
const VIEW_MODEL_RENDER_LAYER: usize = 1;

pub fn spawn_view_model(mut commands: Commands) {
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

pub fn _despawn_world_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(50.0)));
    let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let torus = meshes.add(Torus::new(2.0, 0.5));
    let material = materials.add(Color::WHITE);
    let black_material = materials.add(Color::BLACK);
    commands.spawn((Mesh3d(floor), MeshMaterial3d(material.clone())));

    commands.spawn((
        Mesh3d(cube.clone()),
        MeshMaterial3d(black_material.clone()),
        Transform::from_xyz(1.0, 3.0, 0.0),
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

pub fn spawn_world_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut planet_counter = 0;
    let mut rng = rand::rng();
    let mut planet_coord_vector: Vec<Vec<f32>> = vec![vec![], vec![], vec![]];
    let mut planet_radius_vector: Vec<f32> = vec![];
    loop {
        let random_number_x: f32 = rng.random_range(-50.0..50.0);
        planet_coord_vector[0].push(random_number_x);
        planet_counter += 1;
        if planet_coord_vector[0].len() > 50 {
            break;
        }
    }

    loop {
        let random_number_y: f32 = rng.random_range(-50.0..50.0);
        planet_coord_vector[1].push(random_number_y);
        if planet_coord_vector[1].len() > 50 {
            break;
        }
    }

    loop {
        let random_number_z: f32 = rng.random_range(-50.0..50.0);
        planet_coord_vector[2].push(random_number_z);
        if planet_coord_vector[2].len() > 50 {
            break;
        }
    }

    loop {
        let random_number_r: f32 = rng.random_range(0.0..25.0);
        planet_radius_vector.push(random_number_r);
        if planet_radius_vector.len() > planet_counter {
            break;
        }
    }
    let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let torus = meshes.add(Torus::new(2.0, 0.5));
    let material = materials.add(Color::WHITE);
    let black_material = materials.add(Color::BLACK);

    let mut object_mesh_vector: Vec<Handle<Mesh>> = vec![];
    for radius in planet_radius_vector {
        let planet = meshes.add(Sphere::new(radius));
        object_mesh_vector.push(planet);
    }

    for (ra, rb, rc, planet_object) in planet_coord_vector[0]
        .iter()
        .zip(planet_coord_vector[1].iter())
        .zip(planet_coord_vector[2].iter())
        .zip(object_mesh_vector.iter())
        .map(|(((ra, rb), rc), planet_object)| (ra, rb, rc, planet_object))
    {
        let x = *ra;
        let y = *rb;
        let z = *rc;

        commands.spawn((
            Mesh3d(planet_object.clone()),
            MeshMaterial3d(material.clone()),
            Transform::from_xyz(x, y, z),
        ));
    }

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
        Transform::from_xyz(-5.0, 4.0, -0.75),
        // The light source illuminates both the world model and the view model.
        RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
    ));

    commands.spawn((
        PointLight {
            color: Color::from(tailwind::ROSE_300),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(5.0, 4.0, 0.75),
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
            "Press WASD to move around.\n",
            "Press middle button on the mouse to grab.\n",
            "Press Left moues button to summon a cube.\n",
        )));
}

pub fn spawn_cube(
    _click: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut num: Local<usize>,
    mut query: Query<(&mut Transform, &Player)>,
) {
    if let Ok((transform, _player)) = query.get_single_mut() {
        let location = transform.translation;
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.6, 0.6, 0.6))),
            //99, 144, 255
            MeshMaterial3d(materials.add(Color::srgb_u8(99, 144, 255))),
            Transform::from_xyz(location.x, location.y, location.z),
            //(0.0, 0.25 + 0.55 * *num as f32, 0.0),
        ));
        // With the MeshPickingPlugin added, you can add pointer event observers to meshes:
        *num += 1;
    }
}
