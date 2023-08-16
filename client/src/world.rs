use crate::types::*;
use bevy::log;
use bevy::prelude::*;
use bevy_ggrs::AddRollbackCommandExtension;
use bevy_ggrs::Session;
use ggrs::PlayerHandle;
use std::default::Default;

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

#[derive(Component, Reflect, Default, Debug)]
pub struct Allegiance {
    pub handle: PlayerHandle,
}

#[derive(Component, Reflect, Default)]
pub struct Stocks {
    pub count: u8,
}

// Rather than use a floating-point transform system,
// the game logic uses integers. This is translated to
// floats for the graphics system.
#[derive(Component, Reflect, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

fn posn_to_translation(p: Position) -> Vec2 {
    Vec2::new(p.x as f32, p.y as f32)
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

// The Command is not the final say
// on the behaviour of the character.
// For example, a character who is falling
// and actives RightTilt will do a FAir or BAir
// depending on their orientation.
#[derive(Debug, Reflect)]
pub enum IntentKind {
    GoRight,
    GoLeft,
    Jab,
    RightTilt,
    LeftTilt,
    DownTilt,
    Jump,
    Neutral,
    Crouch,
    CrawlRight,
    CrawlLeft,
}

impl Default for IntentKind {
    fn default() -> Self {
        IntentKind::Neutral
    }
}

#[derive(Component, Default, Reflect, Debug)]
pub struct Intent(pub IntentKind);

pub fn startup_system(mut commands: Commands) {
    let num_players = 2;
    log::debug!("Spawning camera");
    commands.spawn(Camera2dBundle::default());
    log::debug!("Spawning fighters");
    commands
        .spawn((
            Fighter {},
            Allegiance { handle: 0 },
            Intent(IntentKind::Neutral),
            Position { x: -50, y: 0 },
            Velocity { x: 0, y: 0 },
            Acceleration { x: 0, y: 0 },
            Stocks { count: 4 },
            CollisionRect {
                width: 80,
                height: 80,
            },
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(-50., 0., 0.),
                    scale: Vec3::new(20., 20., 20.),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(1., 0.47, 0.),
                    ..default()
                },
                ..default()
            },
        ))
        .add_rollback();
    commands
        .spawn((
            Fighter {},
            Allegiance { handle: 1 },
            Intent(IntentKind::Neutral),
            Position { x: 50, y: 50 },
            Velocity { x: 0, y: 0 },
            Acceleration { x: 0, y: 0 },
            Stocks { count: 4 },
            CollisionRect {
                width: 80,
                height: 80,
            },
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(50., 0., 0.),
                    scale: Vec3::new(20., 20., 20.),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(0., 0.47, 1.),
                    ..default()
                },
                ..default()
            },
        ))
        .add_rollback();
}
