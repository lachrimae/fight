use crate::input::CombinedInput;
use bevy::prelude::*;
use std::collections::HashMap;
use std::net::SocketAddr;

#[derive(Debug)]
pub struct GgrsConfig;

#[derive(Resource, Reflect, Default, Debug)]
pub struct PlayerId(pub usize);

impl ggrs::Config for GgrsConfig {
    type Input = CombinedInput;
    type State = ();
    type Address = SocketAddr;
}
