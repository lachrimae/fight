use bevy::log;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ggrs::AddRollbackCommandExtension;

use std::default::Default;

use crate::types::*;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "postbox-stand.png")]
    pub postbox_stand: Handle<Image>,
    #[asset(path = "postbox-walk.png")]
    pub postbox_walk: Handle<Image>,
    #[asset(path = "postbox-jab.png")]
    pub postbox_jab: Handle<Image>,
}

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

#[derive(Debug, Component)]
pub struct Platform {
    pub x: i32,
    pub y: i32,
    pub width: i32,
}

#[derive(Component, Reflect, Default, Debug)]
pub struct Allegiance {
    pub handle: PlayerId,
}

#[derive(Component, Reflect, Default)]
pub struct Stocks {
    pub count: u8,
}

// Rather than use a floating-point transform system,
// the game logic uses integers. This is translated to
// floats for the graphics system.
#[derive(Debug, Component, Reflect, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

fn posn_to_translation(p: Position) -> Vec2 {
    Vec2::new(p.x as f32, p.y as f32)
}

#[derive(Component, Reflect, Default)]
pub struct Moving {}

#[derive(Debug, Component, Reflect, Default)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Reflect, Default)]
pub struct Accelerating {}

#[derive(Debug, Component, Reflect, Default)]
pub struct Acceleration {
    pub x: i32,
    pub y: i32,
}

// The Command is not the final say
// on the behaviour of the character.
// For example, a character who is falling
// and actives RightTilt will do a FAir or BAir
// depending on their orientation.
#[derive(Debug, Reflect, Default)]
pub enum IntentKind {
    #[default]
    Neutral,
    GoRight,
    GoLeft,
    Jab,
    RightTilt,
    LeftTilt,
    DownTilt,
    Jump,
    Crouch,
    CrawlRight,
    CrawlLeft,
}

#[derive(Component, Default, Reflect, Debug)]
pub struct Intent(pub IntentKind);

#[derive(PartialEq, Eq, Copy, Clone, Debug, Reflect, Default)]
pub enum Orientation {
    Left,
    #[default]
    Right,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Reflect, Default)]
pub struct Jumps(pub u8);

#[derive(PartialEq, Eq, Copy, Clone, Debug, Reflect, Default)]
pub enum Action {
    #[default]
    Standing,
    Jumping(Jumps),
    Falling(Jumps),
    Walking,
    Jabbing,
    NAiring(Jumps),
    //    FAiring,
    //    BAiring,
    //    UpAiring,
    //    DAiring,
    //    FTilting,
    //    UpTilting,
    //    DTilting,
    //    Jabbing,
    //    Crouching,
    //    Crawling,
}

#[derive(Component, Default, Reflect, Debug)]
pub struct FightingStance {
    pub orientation: Orientation,
    pub action: Action,
    pub countdown: i8,
    pub countup: u8,
}

pub fn startup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    log::debug!("Loading sprites");
    let stand_texture = asset_server.load("postbox-stand.png");

    log::debug!("Spawning camera");
    commands.spawn(Camera2dBundle::default());
    log::debug!("Spawning fighters");
    commands.spawn((
        Platform {
            x: -50,
            y: 0,
            width: 100,
        },
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
            sprite: Sprite {
                color: Color::rgb(0., 0., 0.),
                custom_size: Some(Vec2::new(100., 1.)),
                ..default()
            },
            ..default()
        },
    ));
    commands
        .spawn((
            Fighter {},
            Allegiance {
                handle: PlayerId(0),
            },
            Intent(IntentKind::Neutral),
            FightingStance::default(),
            Position { x: 0, y: 40 },
            Velocity { x: 0, y: 0 },
            Acceleration { x: 0, y: 0 },
            Accelerating {},
            Moving {},
            Stocks { count: 4 },
            CollisionRect {
                width: 80,
                height: 80,
            },
            SpriteBundle {
                texture: stand_texture,
                ..default()
            },
        ))
        .add_rollback();
}
