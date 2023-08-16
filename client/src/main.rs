use bevy::prelude::*;
use fight_common::*;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_systems(Startup, startup_system)
        .add_systems(FixedUpdate, movement_system)
        .add_systems(FixedUpdate, acceleration_system)
        .insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP))
        .run();
}
