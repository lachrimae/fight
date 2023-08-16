use bevy::prelude::*;

#[derive(Component)]
pub struct Fighter {}

#[derive(Component)]
pub struct Collides {}

#[derive(Component)]
pub struct DoesDamage {}

#[derive(Component)]
pub struct Environment {}

#[derive(Component)]
pub struct CollisionCirc {
    pub radius: i32,
}

#[derive(Component)]
pub struct Allegiance {
    pub player_id: u8,
}

#[derive(Component)]
pub struct Stocks {
    pub count: u8,
}

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Moving {}

#[derive(Component)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Accelerating {}

#[derive(Component)]
pub struct Acceleration {
    pub x: i32,
    pub y: i32,
}

pub fn acceleration_system(mut query: Query<(&mut Velocity, &Acceleration), With<Accelerating>>) {
    for (mut velocity, acceleration) in &mut query {
        velocity.x += acceleration.x;
        velocity.y += acceleration.y;
    }
}

pub fn movement_system(mut query: Query<(&mut Position, &Velocity), With<Moving>>) {
    for (mut position, velocity) in &mut query {
        position.x += velocity.x;
        position.y += velocity.y;
    }
}

pub fn startup_system(mut commands: Commands) {
    commands.spawn_batch(vec![
        (
            Fighter {},
            Allegiance { player_id: 0 },
            Position { x: -50, y: 0 },
            Velocity { x: 0, y: 0 },
            Acceleration { x: 0, y: 0 },
            Stocks { count: 4 },
            Collides {},
        ),
        (
            Fighter {},
            Allegiance { player_id: 1 },
            Position { x: 50, y: 50 },
            Velocity { x: 0, y: 0 },
            Acceleration { x: 0, y: 0 },
            Stocks { count: 4 },
            Collides {},
        ),
    ]);
}

pub const FIXED_TIMESTEP: f32 = 0.5;
