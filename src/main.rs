mod components;
mod experiments;
mod player;

use crate::components::rapier_helpers::*;
use crate::components::SmallBox;
use bevy::asset::ChangeWatcher;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_fps_controller::controller::*;
use bevy_proto::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_sprite3d::Sprite3dPlugin;
use player::player_components::*;
use player::LocomotionPlugin;
use std::time::Duration;

fn main() {
    color_eyre::install().unwrap();

    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // Enable hot-reloading of assets:
            watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(EditorPlugin::default())
        .add_plugins(ProtoPlugin::default())
        .add_plugins(FpsControllerPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(LocomotionPlugin)
        .add_plugins(Sprite3dPlugin)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.5,
        })
        .add_systems(Startup, load)
        .add_systems(Startup, setup)
        .add_plugins(RapierHelperPlugin)
        .register_type::<SmallBox>()
        .register_type::<Playable>()
        .run();
}

// A schematic can be pretty much anything that mutates the world.
// The simplest type of a schematic is just a regular Bevy component.
// For components, we can simply add the `Schematic` derive:
#[derive(Component, Schematic)]
// First thing's first, we need to derive `Reflect` so that we can register
// this type to the registry (speaking of, don't forget to do that!):
#[derive(Reflect)]
// Lastly, we need to register `ReflectSchematic`, which can do like this:
#[reflect(Schematic)]
struct Playable;

#[derive(Component, Default, Reflect)]
pub struct MainCamera;

fn load(mut prototypes: PrototypesMut) {
    prototypes.load_folder("prefabs").unwrap();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let plane_size = Vec3::new(100.0, 0.1, 100.0);
    // plane
    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            Collider::cuboid(plane_size.x * 0.5, plane_size.y * 0.5, plane_size.z * 0.5),
            RigidBody::Fixed,
        ))
        .with_children(|commands| {
            commands.spawn(PbrBundle {
                transform: Transform::from_xyz(0.0, plane_size.y * 0.5, 0.0),
                mesh: meshes.add(shape::Plane::from_size(plane_size.x).into()),
                material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                ..default()
            });
        });

    // spawn_small_box(&mut commands, asset_server);
    spawn_blaster(Vec3::splat(3.0), &mut commands, asset_server.as_ref());
    spawn_small_box(Vec3::splat(1.0), &mut commands, asset_server.as_ref());
    spawn_small_box(Vec3::splat(2.0), &mut commands, asset_server.as_ref());

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            },
            RenderPlayer(0),
            MainCamera,
        ))
        .with_children(|commands| {
            commands.spawn((
                PbrBundle {
                    transform: Transform::from_xyz(0.5, -0.3, -0.9),
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
                RightHand,
                Name::new("right_hand"),
                Grabber { ..default() },
            ));

            commands.spawn((
                PbrBundle {
                    transform: Transform::from_xyz(-0.5, -0.3, -0.9),
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
                LeftHand,
                Name::new("left_hand"),
                Grabber { ..default() },
            ));
        });
}
#[allow(dead_code)]
fn spawn_small_box(position: Vec3, commands: &mut Commands, asset_server: &AssetServer) {
    let box_gltf = asset_server.load("models/box-small.glb#Scene0");
    let size = 0.25;
    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_translation(position),
                ..Default::default()
            },
            Collider::cuboid(size, size, size),
            RigidBody::Dynamic,
            Name::new("little_cube"),
            CubeCollider {
                size: Vec3::splat(size),
            },
            Grabbable,
        ))
        .with_children(|commands| {
            commands.spawn(SceneBundle {
                scene: box_gltf,
                transform: Transform::from_xyz(0.0, -0.25, 0.0),
                ..default()
            });
        });
}

fn spawn_blaster(position: Vec3, commands: &mut Commands, asset_server: &AssetServer) {
    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_translation(position),
                ..Default::default()
            },
            RigidBody::Dynamic,
            Name::new("blaster"),
            Grabbable,
        ))
        .with_children(|commands| {
            commands.spawn(SceneBundle {
                scene: asset_server.load("models/blasterB.glb#Scene0"),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            });
            commands.spawn((
                Name::new("top_collider"),
                SpatialBundle {
                    transform: Transform::from_xyz(0.0, 0.04, 0.0),
                    ..default()
                },
                // CubeGizmo {
                //     color: Color::GREEN,
                // },
                Collider::cuboid(0.1 * 0.5, 0.17 * 0.5, 0.41 * 0.5),
                // Grabbable,
            ));
            commands.spawn((
                SpatialBundle {
                    transform: Transform::from_xyz(0.0, -0.08, -0.13),
                    ..default()
                },
                Collider::cuboid(0.1 * 0.5, 0.13 * 0.5, 0.13 * 0.5),
                // Grabbable, // CubeGizmo {
                //     color: Color::GREEN,
                // },
            ));
        });
}
