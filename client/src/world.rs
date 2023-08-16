use bevy::log;
use bevy::prelude::*;

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
    pub player_id: u8,
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

pub fn startup_system(mut commands: Commands) {
    log::debug!("Spawning fighters");
    commands.spawn_batch(vec![
        (
            Fighter {},
            Allegiance { player_id: 0 },
            Position { x: -50, y: 0 },
            Velocity { x: 0, y: 0 },
            Acceleration { x: 0, y: 0 },
            Stocks { count: 4 },
            CollisionRect {
                width: 80,
                height: 80,
            },
        ),
        (
            Fighter {},
            Allegiance { player_id: 1 },
            Position { x: 50, y: 50 },
            Velocity { x: 0, y: 0 },
            Acceleration { x: 0, y: 0 },
            Stocks { count: 4 },
            CollisionRect {
                width: 80,
                height: 80,
            },
        ),
    ]);
}
