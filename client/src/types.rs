use crate::input::CombinedInput;
use std::net::SocketAddr;

#[derive(Debug)]
pub struct GgrsConfig;

impl ggrs::Config for GgrsConfig {
    type Input = CombinedInput;
    type State = u8;
    type Address = SocketAddr;
}
