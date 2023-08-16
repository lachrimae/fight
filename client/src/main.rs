#![feature(hash_set_entry)]

use bevy::log;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_ggrs::{GgrsAppExtension, GgrsPlugin, GgrsSchedule, Session};
use ggrs::{GGRSEvent as GgrsEvent, PlayerType, SessionBuilder, UdpNonBlockingSocket};

use std::net::SocketAddr;
use std::str::FromStr;

const FPS: usize = 60;

mod input;
mod intent;
mod types;
mod world;

#[derive(Resource)]
struct NetworkStatsTimer(Timer);

fn main() {
    let mut sess_build = SessionBuilder::<types::GgrsConfig>::new()
        .with_num_players(2)
        .with_desync_detection_mode(ggrs::DesyncDetection::On { interval: 10 })
        .with_max_prediction_window(12)
        .with_input_delay(2);

    sess_build = sess_build.add_player(PlayerType::Local, 0).unwrap();
    // TODO: figure out what third argument does
    sess_build = sess_build
        .add_player(
            PlayerType::Remote(SocketAddr::from_str("127.0.0.1:3002").unwrap()),
            1,
        )
        .unwrap();
    let socket = UdpNonBlockingSocket::bind_to_port(5005).unwrap();
    let sess = sess_build.start_p2p_session(socket).unwrap();

    let mut app = App::new();
    log::info!("Configuring Bevy app");
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(720., 720.),
            title: "Fight!".to_owned(),
            ..default()
        }),
        ..default()
    }))
    .add_ggrs_plugin(
        GgrsPlugin::<types::GgrsConfig>::new()
            .with_update_frequency(FPS)
            .with_input_system(input::input_system)
            .register_rollback_component::<world::Intent>()
            .register_rollback_component::<world::Allegiance>(),
    )
    .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
    .insert_resource(Session::P2P(sess))
    .add_systems(Startup, world::startup_system)
    .add_systems(
        GgrsSchedule,
        intent::set_intent_system,
        // movement_system.after(set_intent_system),
        // acceleration_system.after(movement_system),
    )
    .run();
}
