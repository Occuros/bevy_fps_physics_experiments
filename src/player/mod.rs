pub mod player_components;
pub mod player_systems;

use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy_xpbd_3d::prelude::*;

use crate::{general::fixed_joint_with_rotation::FixedJointWithRotation, MainCamera};

use self::{
    player_components::{Grabbable, Grabber, PIDController, RightHand},
    player_systems::move_player_system,
};

pub struct LocomotionPlugin;

const SPAWN_POINT: Vec3 = Vec3::new(0.0, 1.0, 0.0);

impl Plugin for LocomotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, respawn)
            .add_systems(Update, manage_cursor)
            .add_systems(PostUpdate, draw_crossair)
            .add_systems(Update, grabber_target_checking_system)
            .add_systems(Update, grabbing_system)
            .add_systems(
                Update,
                right_hand_placement_system.after(move_player_system),
            )
            .add_systems(Update, move_player_system)
            .register_type::<Grabber>()
            .register_type::<PIDController>();
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
    // commands
    //     .spawn((
    //         Collider::capsule(Vec3::Y * 0.5, Vec3::Y * 1.5, 0.25),
    //         Friction {
    //             coefficient: 0.0,
    //             combine_rule: CoefficientCombineRule::Min,
    //         },
    //         Restitution {
    //             coefficient: 0.0,
    //             combine_rule: CoefficientCombineRule::Min,
    //         },
    //         ActiveEvents::COLLISION_EVENTS,
    //         Velocity::zero(),
    //         RigidBody::Dynamic,
    //         Sleeping::disabled(),
    //         LockedAxes::ROTATION_LOCKED,
    //         AdditionalMassProperties::Mass(1.0),
    //         GravityScale(0.0),
    //         Ccd { enabled: true }, // Prevent clipping when going fast
    //         TransformBundle::from_transform(Transform::from_translation(SPAWN_POINT)),
    //         LogicalPlayer(0),
    //         FpsControllerInput {
    //             pitch: -TAU / 12.0,
    //             yaw: TAU * 5.0 / 8.0,
    //             ..default()
    //         },
    //         FpsController { ..default() },
    //     ))
    //     .insert((
    //         PbrBundle {
    //             transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //             mesh: meshes.add(
    //                 shape::Icosphere {
    //                     radius: 0.1,
    //                     ..default()
    //                 }
    //                 .try_into()
    //                 .unwrap(),
    //             ),
    //             material: materials.add(Color::BLUE.into()),
    //             ..default()
    //         },
    //         Name::new("Player"),
    //         Player,
    //     ));

    commands.spawn((
        PbrBundle {
            transform: Transform::from_xyz(0.5, 1.3, -0.9),
            mesh: meshes.add(
                shape::Icosphere {
                    radius: 0.1,
                    ..default()
                }
                .try_into()
                .unwrap(),
            ),
            material: materials.add(Color::ORANGE.into()),
            ..default()
        },
        RightHand {
            camera_offset: Vec3::new(0.5, -0.3, -1.9),
        },
        Name::new("right_hand"),
        Grabber {
            grabbing_speed: 1000.0,
            ..default()
        },
        RigidBody::Kinematic,
    ));
}

fn right_hand_placement_system(
    camera_query: Query<&Transform, With<MainCamera>>,
    mut right_hand_query: Query<(&mut Transform, &RightHand), Without<MainCamera>>,
) {
    if camera_query.get_single().is_err() {
        return;
    }
    let camera = camera_query.get_single().unwrap();
    if right_hand_query.get_single().is_err() {
        return;
    }
    let (mut right_hand_transform, right_hand) = right_hand_query.get_single_mut().unwrap();

    right_hand_transform.translation = camera.transform_point(right_hand.camera_offset);
    right_hand_transform.look_to(camera.forward(), Vec3::Y);
}

fn respawn(mut query: Query<(&mut Transform, &mut LinearVelocity)>) {
    for (mut transform, mut velocity) in &mut query {
        if transform.translation.y > -50.0 {
            continue;
        }

        velocity.0 = Vec3::ZERO;
        transform.translation = SPAWN_POINT;
    }
}

fn manage_cursor(
    // btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
    mut window_query: Query<&mut Window>,
) {
    let mut window = window_query.single_mut();
    if key.just_pressed(KeyCode::L) {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    }
    if key.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
}

pub fn grabber_target_checking_system(
    spatial_query: SpatialQuery,
    camera_query: Query<&Transform, With<MainCamera>>,
    mut right_hand_query: Query<&mut Grabber, With<RightHand>>,
    grabbable_query: Query<Entity, With<Grabbable>>,
    transform_query: Query<&Transform>,
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
    let max_toi = 30.0;

    grabber.potential_target = None;
    spatial_query.ray_hits_callback(
        ray_origin,
        ray_dir,
        max_toi,
        true,
        SpatialQueryFilter::default(),
        |hit| {
            let root_entity = hit.entity;
            if grabbable_query.get(root_entity).is_err() {
                return true;
            }
            grabber.potential_target = Some(root_entity);
            return false;
        },
    );
}

pub fn grabbing_system(
    mut commands: Commands,
    mouse: Res<Input<MouseButton>>,
    mut grabber_query: Query<(Entity, &mut Grabber, &GlobalTransform)>,
    mut grabbable_query: Query<
        (
            &mut LinearVelocity,
            &mut AngularVelocity,
            &mut Transform,
            &mut PIDController,
        ),
        With<Grabbable>,
    >,
    time: Res<Time>,
    // player_query: Query<Entity, With<MainCamera>>,
) {
    // if player_query.get_single().is_err() {
    //     return;
    // }
    for (grabber_entity, mut grabber, grabber_transform) in grabber_query.iter_mut() {
        if grabber.grabbed_entity.is_some() {
            let (mut linear, mut ang, _, _) = grabbable_query
                .get_mut(grabber.grabbed_entity.unwrap())
                .unwrap();

            if linear.0.length() > 10.0 {
                linear.0 = Vec3::ZERO;
            }

            //prevent insane spinning
            if ang.0.length() > 1.0 {
                ang.0 = Vec3::ZERO;
            }
            if mouse.pressed(MouseButton::Right) {
                commands
                    .get_entity(grabber.joint.unwrap())
                    .unwrap()
                    .despawn_recursive();

                commands
                    .get_entity(grabber.grabbed_entity.unwrap())
                    .unwrap()
                    .remove::<SleepingDisabled>();
                grabber.grabbed_entity = None;
                grabber.joint = None;
            }
            continue;
        }

        if !mouse.pressed(MouseButton::Left) {
            grabber.attracted_target = None;
            continue;
        };

        if grabber.attracted_target.is_none() && grabber.potential_target.is_none() {
            continue;
        }
        if grabber.attracted_target.is_none() && grabber.potential_target.is_some() {
            grabber.attracted_target = Some(grabber.potential_target.unwrap())
        }

        if grabber.attracted_target.is_none() {
            continue;
        }

        let grabbable_entity = grabber.attracted_target.unwrap();

        let grabbable = grabbable_query.get_mut(grabbable_entity);
        if grabbable.is_err() {
            continue;
        }
        let (mut velocity, mut angular_velocity, mut transform, _) = grabbable.unwrap();

        let direction = grabber_transform.translation() - transform.translation;
        velocity.0 = direction.normalize() * time.delta_seconds() * grabber.grabbing_speed;
        transform.rotation = Quat::IDENTITY;
        // let look_direction = direction;
        // let angular_velocity_error = transform.forward().cross(look_direction.normalize());
        // let angular_velocity_correction = pid.update(angular_velocity_error, time.delta_seconds());

        // velocity.angvel = angular_velocity_correction;
        if direction.length() < 1.0 {
            velocity.0 = Vec3::ZERO;
            angular_velocity.0 = Vec3::ZERO;
            // let joint =
            //     FixedJoint::new(grabber_entity, grabbable_entity).with_local_anchor_1(Vec3::ZERO);

            let joint = FixedJointWithRotation::new(grabber_entity, grabbable_entity)
                .with_local_anchor_1(Vec3::ZERO)
                .with_rotation_1(Quat::from_rotation_y(TAU * 0.5));
            grabber.grabbed_entity = Some(grabbable_entity);
            // commands.get_entity(grabber_entity).unwrap().insert(joint);
            grabber.joint = Some(commands.spawn(joint).id());
            commands
                .get_entity(grabbable_entity)
                .unwrap()
                .insert(SleepingDisabled);
        }
    }
}
