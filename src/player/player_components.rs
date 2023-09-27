use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct LeftHand;

#[derive(Component)]
pub struct RightHand;

#[derive(Component, Reflect, Default)]
pub struct Grabber {
    pub potential_target: Option<Entity>,
    pub grabbed_entity: Option<Entity>,
}

#[derive(Component, Reflect)]
pub struct Grabbable;
