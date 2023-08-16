#![feature(hash_set_entry)]

use bevy::log;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_ggrs::{GgrsAppExtension, GgrsPlugin, GgrsSchedule, Session};
use ggrs::{GGRSEvent as GgrsEvent, PlayerType, SessionBuilder, UdpNonBlockingSocket};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::str::FromStr;

const FPS: usize = 60;

mod input;
mod types;
mod world;

#[derive(Resource)]
struct NetworkStatsTimer(Timer);

#[derive(Debug)]
struct GgrsConfig;

impl ggrs::Config for GgrsConfig {
    type Input = u8;
    type State = u8;
    type Address = SocketAddr;
}

use crate::input::*;
use crate::types::*;
use crate::world::*;

fn main() {
    let mut sess_build = SessionBuilder::<GgrsConfig>::new()
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
    }));
    app.add_ggrs_plugin(
        GgrsPlugin::<GgrsConfig>::new()
            .with_update_frequency(FPS)
            .with_input_system(input)
            .register_rollback_component::<Position>()
            .register_rollback_component::<Velocity>()
            .register_rollback_component::<Acceleration>()
            .register_rollback_component::<Moving>()
            .register_rollback_component::<Accelerating>()
            .register_rollback_component::<Fighter>()
            .register_rollback_component::<Environment>()
            .register_rollback_component::<DoesDamage>()
            .register_rollback_component::<CollisionRect>()
            .register_rollback_component::<Allegiance>()
            .register_rollback_component::<Stocks>(),
    )
    .add_systems(Startup, startup_system)
    .add_systems(
        GgrsSchedule,
        (movement_system, acceleration_system.after(movement_system)),
    )
    .insert_resource(Session::P2P(sess))
    .insert_resource(NetworkStatsTimer(Timer::from_seconds(
        2.0,
        TimerMode::Repeating,
    )))
    .add_systems(Update, print_network_stats_system)
    .add_systems(Update, print_events_system);
    log::info!("Running Bevy app");
    app.run();
}

fn print_events_system(mut session: ResMut<Session<GgrsConfig>>) {
    match session.as_mut() {
        Session::P2P(s) => {
            for event in s.events() {
                match event {
                    GgrsEvent::Disconnected { .. } | GgrsEvent::NetworkInterrupted { .. } => {
                        warn!("GGRS event: {event:?}")
                    }
                    GgrsEvent::DesyncDetected { .. } => error!("GGRS event: {event:?}"),
                    _ => info!("GGRS event: {event:?}"),
                }
            }
        }
        _ => panic!("This example focuses on p2p."),
    }
}

fn print_network_stats_system(
    time: Res<Time>,
    mut timer: ResMut<NetworkStatsTimer>,
    p2p_session: Option<Res<Session<GgrsConfig>>>,
) {
    // print only when timer runs out
    if timer.0.tick(time.delta()).just_finished() {
        if let Some(sess) = p2p_session {
            match sess.as_ref() {
                Session::P2P(s) => {
                    let num_players = s.num_players();
                    for i in 0..num_players {
                        if let Ok(stats) = s.network_stats(i) {
                            println!("NetworkStats for player {}: {:?}", i, stats);
                        }
                    }
                }
                _ => panic!("This examples focuses on p2p."),
            }
        }
    }
}
