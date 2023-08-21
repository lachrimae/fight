#![feature(hash_set_entry)]

use bevy::log;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_asset_loader::prelude::*;
use bevy_fmod::FmodPlugin;
use bevy_ggrs::{GgrsAppExtension, GgrsPlugin, GgrsSchedule, Session};
use ggrs::{PlayerType, SessionBuilder, UdpNonBlockingSocket};

const FPS: usize = 60;

mod action;
mod death;
mod graphics;
mod hud;
mod input;
mod intent;
mod physics;
mod stance;
mod types;
mod world;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
enum GameState {
    #[default]
    AssetLoading,
    InGame,
}

fn main() {
    let mut sess_build = SessionBuilder::<types::GgrsConfig>::new()
        .with_num_players(1)
        .with_desync_detection_mode(ggrs::DesyncDetection::On { interval: 10 })
        .with_max_prediction_window(12)
        .with_input_delay(2);

    sess_build = sess_build.add_player(PlayerType::Local, 0).unwrap();
    // TODO: figure out what third argument does
    // sess_build = sess_build
    //     .add_player(
    //         PlayerType::Remote(SocketAddr::from_str("127.0.0.1:3002").unwrap()),
    //         1,
    //     )
    //     .unwrap();
    let socket = UdpNonBlockingSocket::bind_to_port(5005).unwrap();
    let sess = sess_build.start_p2p_session(socket).unwrap();

    let mut app = App::new();
    log::info!("Configuring Bevy app");
    app.add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::InGame),
        )
        .add_collection_to_loading_state::<_, world::ImageAssets>(GameState::AssetLoading)
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(720., 720.),
                    title: "Fight!".to_owned(),
                    ..default()
                }),
                ..default()
            }),
            FmodPlugin {
                audio_banks_directory: "./fmod/Build/Desktop",
            },
        ))
        .add_ggrs_plugin(
            GgrsPlugin::<types::GgrsConfig>::new()
                .with_update_frequency(FPS)
                .with_input_system(input::input_system)
                .register_rollback_component::<world::CombinedInputDiff>()
                .register_rollback_component::<world::Intent>()
                .register_rollback_component::<world::Allegiance>()
                .register_rollback_component::<world::FightingStance>()
                .register_rollback_component::<world::Velocity>()
                .register_rollback_component::<world::Position>()
                .register_rollback_component::<world::Acceleration>()
                .register_rollback_component::<world::Accelerating>()
                .register_rollback_component::<world::Moving>()
                .register_rollback_component::<world::Stocks>(),
        )
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .insert_resource(types::PlayerId(0))
        // .insert_resource(types::PlayerId(1))
        .insert_resource(Session::P2P(sess))
        .add_systems(OnEnter(GameState::InGame), world::startup_system)
        .add_systems(
            GgrsSchedule,
            (
                intent::input_diff_system,
                intent::set_intent_system,
                stance::set_stance_system,
                physics::set_physical_props_system,
                physics::movement_system,
                physics::acceleration_system,
                death::death_system,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                graphics::update_graphics_system,
                hud::update_stocks,
                hud::update_dmg,
            )
                .run_if(in_state(GameState::InGame)),
        )
        .run();
}
