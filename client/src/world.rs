use bevy::log;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ggrs::AddRollbackCommandExtension;
use bytemuck::{Pod, Zeroable};

use std::default::Default;

use crate::machine::postbox::PostboxState;
use crate::machine::types::{Armour, Physics};
use crate::types::*;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "postbox-stand.png")]
    pub postbox_stand: Handle<Image>,
    #[asset(path = "postbox-walk.png")]
    pub postbox_walk: Handle<Image>,
    #[asset(path = "postbox-jab.png")]
    pub postbox_jab: Handle<Image>,
    #[asset(path = "postbox-nair.png")]
    pub postbox_nair: Handle<Image>,
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

#[derive(Copy, Clone, Default, Reflect, Debug, PartialEq, Eq)]
pub struct PlatformId(u8);

#[derive(Debug, Component)]
pub struct Platform {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub id: PlatformId,
}

const FIGHTER_DIMENSIONS: i32 = 40;

pub fn fighter_is_on_plat(pos: &Position, plat: &Platform) -> bool {
    if pos.x < plat.x + plat.width
        && pos.x + FIGHTER_DIMENSIONS > plat.x
        && pos.y - 1 < plat.y + 1
        && pos.y > plat.y
    {
        log::trace!("Character at {:?} standing on platform at {:?}", pos, plat);
        true
    } else {
        log::trace!(
            "Character at {:?} not standing on platform at {:?}",
            pos,
            plat
        );
        false
    }
}

#[derive(Component, Reflect, Default, Debug)]
pub struct Allegiance {
    pub handle: PlayerId,
}

#[derive(Component, Reflect, Default)]
pub struct Stocks {
    pub count: u8,
}

#[derive(Component, Reflect, Default)]
pub struct Damage {
    pub percent: u16,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum ButtonDiff {
    NotHeld = 0,
    Held = 1,
    Released = 2,
    Pressed = 3,
}

pub fn is_being_pressed(diff: ButtonDiff) -> bool {
    diff == ButtonDiff::Held || diff == ButtonDiff::Pressed
}

#[derive(Default, Component, Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable, Reflect)]
#[repr(C)]
pub struct InputDiff(pub u16);

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

#[derive(Component, PartialEq, Eq, Copy, Clone, Debug, Reflect, Default)]
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

#[derive(Component, Default, Reflect, Debug)]
pub struct StocksText {}

#[derive(Component, Default, Reflect, Debug)]
pub struct DamageText {}

#[derive(Component, Default, Reflect, Debug)]
pub struct StandingOn {
    pub platform: PlatformId,
}

pub fn startup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    log::debug!("Loading sprites");
    let stand_texture = asset_server.load("postbox-stand.png");

    log::debug!("Spawning camera");
    commands.spawn(Camera2dBundle::default());
    log::debug!("Spawning fighters");
    let _main_plat = commands.spawn((
        Platform {
            x: -50,
            y: 0,
            width: 100,
            id: PlatformId(0),
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
            PostboxState::default(),
            InputDiff::default(),
            Physics::default(),
            Armour::default(),
            Orientation::default(),
            Position { x: 0, y: 86 },
            Velocity { x: 0, y: 0 },
            Acceleration { x: 0, y: 0 },
            StandingOn {
                platform: PlatformId(0),
            },
            Stocks { count: 4 },
            Damage { percent: 0 },
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

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(160.),
                    height: Val::Px(80.),
                    border: UiRect::all(Val::Px(2.)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    left: Val::Px(200.),
                    top: Val::Px(640.),
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                background_color: BackgroundColor(Color::WHITE),
                ..default()
            },
            Allegiance {
                handle: PlayerId(0),
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "4 stocks",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 30.0,
                        color: Color::BLACK,
                    },
                ),
                Allegiance {
                    handle: PlayerId(0),
                },
                StocksText {},
            ));
            parent.spawn((
                TextBundle::from_section(
                    "0%",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 30.0,
                        color: Color::BLACK,
                    },
                ),
                Allegiance {
                    handle: PlayerId(0),
                },
                DamageText {},
            ));
        });
}
