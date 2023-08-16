use crate::types::*;
use bevy::log;
use bevy::prelude::*;
use bevy_ggrs::Session;
use ggrs::PlayerHandle;

#[derive(Component, Reflect, Default)]
pub struct Fighter {}

#[derive(Component, Reflect, Default)]
pub struct DoesDamage {}

#[derive(Component, Reflect, Default)]
pub struct Environment {}

#[derive(Component, Reflect, Default)]
pub struct CollisionRect {
    pub width: i32,
    pub height: i32,
}

#[derive(Component, Reflect, Default)]
pub struct Allegiance {
    pub handle: PlayerHandle,
}

#[derive(Component, Reflect, Default)]
pub struct Stocks {
    pub count: u8,
}

#[derive(Component, Reflect, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Reflect, Default)]
pub struct Moving {}

#[derive(Component, Reflect, Default)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Reflect, Default)]
pub struct Accelerating {}

#[derive(Component, Reflect, Default)]
pub struct Acceleration {
    pub x: i32,
    pub y: i32,
}

pub fn movement_system(mut query: Query<(&mut Position, &Velocity), With<Moving>>) {
    log::debug!("movement system beginning");
    for (mut position, velocity) in &mut query {
        position.x += velocity.x;
        position.y += velocity.y;
    }
}

pub fn acceleration_system(mut query: Query<(&mut Velocity, &Acceleration), With<Accelerating>>) {
    log::debug!("acceleration system beginning");
    for (mut velocity, acceleration) in &mut query {
        velocity.x += acceleration.x;
        velocity.y += acceleration.y;
    }
}

pub fn startup_system(
    mut commands: Commands,
    session: Res<Session<GgrsConfig>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let num_players = 2;
    log::debug!("Spawning fighters");
    commands.spawn_batch(vec![
        (
            Fighter {},
            Allegiance { handle: 0 },
            Position { x: -50, y: 0 },
            Velocity { x: 0, y: 0 },
            Acceleration { x: 0, y: 0 },
            Stocks { count: 4 },
            CollisionRect {
                width: 80,
                height: 80,
            },
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 80. })),
                material: materials.add(Color::rgb(0.9, 0.2, 0.2).into()),
                ..default()
            },
        ),
        (
            Fighter {},
            Allegiance { handle: 1 },
            Position { x: 50, y: 50 },
            Velocity { x: 0, y: 0 },
            Acceleration { x: 0, y: 0 },
            Stocks { count: 4 },
            CollisionRect {
                width: 80,
                height: 80,
            },
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 80. })),
                material: materials.add(Color::rgb(0., 0.35, 0.8).into()),
                ..default()
            },
        ),
    ]);
}
