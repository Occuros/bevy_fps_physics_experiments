use crate::player::player_components::*;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use std::f32::consts::{FRAC_PI_2, PI, TAU};

const ANGLE_EPSILON: f32 = 0.001953125;

pub fn move_player_system(
    mut mouse_events: EventReader<MouseMotion>,
    mut input_query: Query<(&mut PlayerInput, &mut Transform)>,
) {
    let sensitivity = 0.001;
    if input_query.get_single().is_err() {
        return;
    }
    let (mut input, mut transform) = input_query.single_mut();

    let mut mouse_delta = Vec2::ZERO;
    for mouse_event in mouse_events.iter() {
        mouse_delta += mouse_event.delta;
    }
    mouse_delta *= sensitivity;

    input.pitch =
        (input.pitch - mouse_delta.y).clamp(-FRAC_PI_2 + ANGLE_EPSILON, FRAC_PI_2 - ANGLE_EPSILON);
    input.yaw -= mouse_delta.x;
    if input.yaw.abs() > PI {
        input.yaw = input.yaw.rem_euclid(TAU);
    }

    transform.rotation = Quat::from_euler(EulerRot::YXZ, input.yaw, input.pitch, 0.0);
}
