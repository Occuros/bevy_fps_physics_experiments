use bevy::prelude::*;
use bevy_proto::prelude::*;

#[derive(Component, Schematic, Reflect)]
#[reflect(Schematic)]
pub struct SmallBox {}
