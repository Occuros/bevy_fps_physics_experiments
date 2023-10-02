use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
// use std::f32::consts::TAU;

#[derive(Component, Default, Reflect)]
pub struct Thing1;

const SPAWN_POINT: Vec3 = Vec3::new(3.0, 2.0, 5.0);

pub fn spawn_experiment(mut commands: Commands, asset_server: Res<AssetServer>) {
    let box_gltf = asset_server.load("models/box-small.glb#Scene0");
    let size = 0.25;

    let cube_1_entity = commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_translation(SPAWN_POINT),
                ..Default::default()
            },
            // Collider::cuboid(size, size, size),
            RigidBody::Kinematic,
            Name::new("little_cube"),
            Thing1,
        ))
        .with_children(|commands| {
            commands.spawn(SceneBundle {
                scene: box_gltf.clone_weak(),
                transform: Transform::from_xyz(0.0, -0.25, 0.0),
                ..default()
            });
        })
        .id();

    let cube_2_entity = commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_translation(Vec3::new(2.0, 2.0, 2.0)),
                ..Default::default()
            },
            Collider::cuboid(size, size, size),
            RigidBody::Dynamic,
            Name::new("little_cube"),
        ))
        .with_children(|commands| {
            commands.spawn(SceneBundle {
                scene: box_gltf,
                transform: Transform::from_xyz(0.0, -0.25, 0.0),
                ..default()
            });
        })
        .id();

    commands.spawn(
        FixedJoint::new(cube_1_entity, cube_2_entity).with_local_anchor_1(Vec3::new(1.5, 0.0, 0.0)),
    );
}

pub fn rotate_thing_1(mut thing1_query: Query<&mut Transform, With<Thing1>>, time: Res<Time>) {
    for mut t in thing1_query.iter_mut() {
        let position = SPAWN_POINT + Vec3::X * 1.0;
        let rotation = Quat::from_rotation_y(time.delta_seconds() / 5.2);
        t.rotate_around(position, rotation);

        // t.rotate_y(TAU * 0.1 * time.delta_seconds());
    }
}
