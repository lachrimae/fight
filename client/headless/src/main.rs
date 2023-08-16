use bevy::log;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use fight_common::*;

mod app;

fn main() {
    let mut app = App::new();
    app.add_plugin(LogPlugin::default());
    log::debug!("Adding minimal plugins");
    app.add_plugins(MinimalPlugins);
    log::debug!("Adding startup system");
    app.add_systems(Startup, startup_system);
    log::debug!("Adding fixed update system");
    app.add_systems(FixedUpdate, (movement_system, acceleration_system).chain());
    app.insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP));
    log::info!("Starting app");
    app.run();
}
