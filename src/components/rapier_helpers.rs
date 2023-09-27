use bevy::prelude::*;
use bevy_proto::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct RapierHelperPlugin;

impl Plugin for RapierHelperPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ColliderProto>()
            .register_type::<RigidbodyProto>()
            .register_type::<CubeCollider>()
            .register_type::<CubeGizmo>()
            .add_systems(Update, cube_collider_changed)
            .add_systems(Update, draw_cube_gizmo);
    }
}
#[derive(Component, Default, Debug, Reflect)]
pub struct CubeGizmo {
    pub color: Color,
}

pub fn draw_cube_gizmo(
    cube_gizmo_query: Query<(&GlobalTransform, &CubeGizmo)>,
    mut gizmos: Gizmos,
) {
    for (transform, cube_gizmo) in cube_gizmo_query.iter() {
        gizmos.cuboid(
            Transform::from_matrix(transform.compute_matrix()),
            cube_gizmo.color,
        );
    }
}

#[derive(Component, Default, Debug, Reflect)]
pub struct CubeCollider {
    pub size: Vec3,
}

fn cube_collider_changed(
    mut commands: Commands,
    mut collider_query: Query<(Entity, &CubeCollider), Changed<CubeCollider>>,
) {
    for (entity, cube_collider) in collider_query.iter_mut() {
        let size = cube_collider.size;
        let col = Collider::cuboid(size.x, size.y, size.z);
        commands.entity(entity).insert(col);
    }
}

#[derive(Schematic, Reflect, Debug)]
#[reflect(Schematic)]
#[schematic(into = RigidBody)]
pub enum RigidbodyProto {
    Dynamic,
    Fixed,
}

impl From<RigidbodyProto> for RigidBody {
    fn from(value: RigidbodyProto) -> Self {
        match value {
            RigidbodyProto::Dynamic => RigidBody::Dynamic,
            RigidbodyProto::Fixed => RigidBody::Fixed,
        }
    }
}

#[derive(Schematic, Reflect)]
#[reflect(Schematic)]
#[schematic(into = Collider)]
pub struct ColliderProto {
    x: f32,
    y: f32,
    z: f32,
}

impl From<ColliderProto> for Collider {
    fn from(col_state: ColliderProto) -> Collider {
        Collider::cuboid(col_state.x, col_state.y, col_state.z)
    }
}
