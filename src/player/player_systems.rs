use crate::player::player_components::*;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use std::f32::consts::{FRAC_PI_2, PI, TAU};

const ANGLE_EPSILON: f32 = 0.001953125;

pub fn move_player_system(
    mut mouse_events: EventReader<MouseMotion>,
    key_input: Res<Input<KeyCode>>,
    mut input_query: Query<(&mut PlayerInput, &mut Transform)>,
    time: Res<Time>,
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

    input.movement = Vec3::new(
        get_axis(&key_input, KeyCode::D, KeyCode::A),
        0.0,
        get_axis(&key_input, KeyCode::W, KeyCode::S),
    );

    let mut movement =
        input.movement.x * transform.right() + input.movement.z * transform.forward();
    movement.y = 0.0;

    transform.rotation = Quat::from_euler(EulerRot::YXZ, input.yaw, input.pitch, 0.0);
    transform.translation += movement.normalize_or_zero() * time.delta_seconds() * 10.0;
}

fn get_axis(key_input: &Res<Input<KeyCode>>, key_pos: KeyCode, key_neg: KeyCode) -> f32 {
    get_pressed(key_input, key_pos) - get_pressed(key_input, key_neg)
}

fn get_pressed(key_input: &Res<Input<KeyCode>>, key: KeyCode) -> f32 {
    if key_input.pressed(key) {
        1.0
    } else {
        0.0
    }
}
