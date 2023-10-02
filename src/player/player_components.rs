use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct LeftHand;

#[derive(Component, Default)]
pub struct RightHand {
    pub camera_offset: Vec3,
}

#[derive(Component, Reflect, Default)]
pub struct Grabber {
    pub grabbing_speed: f32,
    pub potential_target: Option<Entity>,
    pub attracted_target: Option<Entity>,
    pub grabbed_entity: Option<Entity>,
    pub joint: Option<Entity>,
}

#[derive(Component, Reflect)]
pub struct Grabbable;

#[derive(Component, Default, Reflect)]
pub struct PIDController {
    pub p_factor: f32,
    pub i_factor: f32,
    pub d_factor: f32,
    integral: Vec3,
    last_error: Vec3,
}

#[allow(dead_code)]
impl PIDController {
    pub fn new(p_factor: f32, i_factor: f32, d_factor: f32) -> Self {
        Self {
            p_factor,
            i_factor,
            d_factor,
            integral: Vec3::ZERO,
            last_error: Vec3::ZERO,
        }
    }

    pub fn update(&mut self, current_error: Vec3, delat_time: f32) -> Vec3 {
        self.integral += current_error * delat_time;
        let derivative = (current_error - self.last_error) / delat_time;
        self.last_error = current_error;
        return current_error * self.p_factor
            + self.integral * self.i_factor
            + derivative * self.d_factor;
    }
}

#[derive(Component, Default, Reflect)]
pub struct PlayerInput {
    pub fly: bool,
    pub sprint: bool,
    pub jump: bool,
    pub crouch: bool,
    pub pitch: f32,
    pub yaw: f32,
    pub movement: Vec3,
}
