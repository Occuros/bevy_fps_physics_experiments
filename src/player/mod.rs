pub mod player_components;

use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy_fps_controller::controller::{FpsController, FpsControllerInput, LogicalPlayer};
use bevy_rapier3d::prelude::*;
use std::f32::consts::TAU;

use crate::player::player_components::Player;
use crate::MainCamera;

use self::player_components::{Grabbable, Grabber, RightHand};

pub struct LocomotionPlugin;

const SPAWN_POINT: Vec3 = Vec3::new(0.0, 1.0, 0.0);

impl Plugin for LocomotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, respawn)
            .add_systems(Update, manage_cursor)
            .add_systems(PostUpdate, draw_crossair)
            .add_systems(Update, grabber_target_checking_system)
            .register_type::<Grabber>();
    }
}

fn draw_crossair(mut gizmos: Gizmos, camera_transform_query: Query<&Transform, With<MainCamera>>) {
    if camera_transform_query.get_single().is_err() {
        return;
    };
    let corsair_size = 0.01;
    let color = Color::WHITE;
    let t = camera_transform_query.get_single().unwrap();
    let start = t.translation + t.forward() - t.right() * corsair_size;
    // let start = Vec3::ZERO;
    let end = t.translation + t.forward() + t.right() * corsair_size;

    gizmos.line(start, end, color);

    let top = t.translation + t.forward() + t.up() * corsair_size;
    let bottom = t.translation + t.forward() - t.up() * corsair_size;
    gizmos.line(top, bottom, color);
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            Collider::capsule(Vec3::Y * 0.5, Vec3::Y * 1.5, 0.5),
            Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            ActiveEvents::COLLISION_EVENTS,
            Velocity::zero(),
            RigidBody::Dynamic,
            Sleeping::disabled(),
            LockedAxes::ROTATION_LOCKED,
            AdditionalMassProperties::Mass(1.0),
            GravityScale(0.0),
            Ccd { enabled: true }, // Prevent clipping when going fast
            TransformBundle::from_transform(Transform::from_translation(SPAWN_POINT)),
            LogicalPlayer(0),
            FpsControllerInput {
                pitch: -TAU / 12.0,
                yaw: TAU * 5.0 / 8.0,
                ..default()
            },
            FpsController { ..default() },
        ))
        .insert((
            PbrBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                mesh: meshes.add(
                    shape::Icosphere {
                        radius: 0.1,
                        ..default()
                    }
                    .try_into()
                    .unwrap(),
                ),
                material: materials.add(Color::BLUE.into()),
                ..default()
            },
            Name::new("Player"),
            Player,
        ));

    // commands.spawn((
    //     PbrBundle {
    //         transform: Transform::from_xyz(-0.3, 1.5, 1.10),
    //         mesh: meshes.add(
    //             shape::Icosphere {
    //                 radius: 0.1,
    //                 ..default()
    //             }
    //             .try_into()
    //             .unwrap(),
    //         ),
    //         material: materials.add(Color::PINK.into()),
    //         ..default()
    //     },
    //     RightHand,
    //     Name::new("right_hand"),
    // ));

    commands.spawn((
        PbrBundle {
            transform: Transform::from_xyz(-0.3, -0.5, 0.10),
            mesh: meshes.add(
                shape::Icosphere {
                    radius: 0.1,
                    ..default()
                }
                .try_into()
                .unwrap(),
            ),
            material: materials.add(Color::PINK.into()),
            ..default()
        },
        RightHand,
    ));
}

fn respawn(mut query: Query<(&mut Transform, &mut Velocity)>) {
    for (mut transform, mut velocity) in &mut query {
        if transform.translation.y > -50.0 {
            continue;
        }

        velocity.linvel = Vec3::ZERO;
        transform.translation = SPAWN_POINT;
    }
}

fn manage_cursor(
    // btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
    mut window_query: Query<&mut Window>,
    mut controller_query: Query<&mut FpsController>,
) {
    let mut window = window_query.single_mut();
    if key.just_pressed(KeyCode::L) {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
        for mut controller in &mut controller_query {
            controller.enable_input = true;
        }
    }
    if key.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
        for mut controller in &mut controller_query {
            controller.enable_input = false;
        }
    }
}

pub fn grabber_target_checking_system(
    rapier_context: Res<RapierContext>,
    camera_query: Query<&Transform, With<MainCamera>>,
    mut right_hand_query: Query<&mut Grabber, With<RightHand>>,
    grabbable_query: Query<Entity, With<Grabbable>>,
    transform_query: Query<&Transform>,
    mouse_input: Res<Input<MouseButton>>,
    mut gizmos: Gizmos,
) {
    if camera_query.get_single().is_err() {
        return;
    }

    if right_hand_query.get_single().is_err() {
        return;
    }

    let mut grabber = right_hand_query.get_single_mut().unwrap();

    if grabber.grabbed_entity.is_some() {
        return;
    }
    let camera = camera_query.get_single().unwrap();

    if let Some(potential_entity) = grabber.potential_target {
        if let Ok(transform) = transform_query.get(potential_entity) {
            let position = (transform.translation - camera.translation) * 0.5 + camera.translation;
            gizmos.circle(position, -camera.forward(), 0.1, Color::WHITE);
        }
    }

    let ray_origin = camera.translation;
    let ray_dir = camera.forward();
    let max_toi = 10.0;

    grabber.potential_target = None;
    rapier_context.intersections_with_ray(
        ray_origin,
        ray_dir,
        max_toi,
        true,
        QueryFilter::default(),
        |entity, _| {
            let root_entity = rapier_context.collider_parent(entity);
            if root_entity.is_none() {
                return true;
            };
            let root_entity = root_entity.unwrap();
            if grabbable_query.get(root_entity).is_err() {
                return true;
            }
            grabber.potential_target = Some(root_entity);
            return false;
        },
    );
}
