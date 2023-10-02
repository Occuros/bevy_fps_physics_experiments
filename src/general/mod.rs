use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use bevy_xpbd_3d::{SubstepSchedule, SubstepSet};

use self::fixed_joint_with_rotation::FixedJointWithRotation;

pub mod fixed_joint_with_rotation;

pub struct GeneralPlugin;

impl Plugin for GeneralPlugin {
    fn build(&self, app: &mut App) {
        let substeps = app
            .get_schedule_mut(SubstepSchedule)
            .expect("add SubstepSchedule first");

        substeps.add_systems(
            (solve_constraint::<FixedJointWithRotation, 2>,)
                .in_set(SubstepSet::SolveUserConstraints),
        );

        // substeps.add_systems(
        //     (joint_damping::<FixedJointWithRotation>,)
        //         .chain()
        //         .in_set(SubstepSet::SolveVelocities),
        // );
    }
}
