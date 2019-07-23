mod app;

use tcod::map::{FovAlgorithm, Map as FovMap};
use tcod::colors::{ Color};
use tcod::console::*;
use app::data::{ Tcod };

#[macro_use]
extern crate serde_derive;

// CONSTANTS
const LIMIT_FPS: i32 = 20; // main
const PLAYER: usize = 0; // everywhere

const FOV_ALGO: FovAlgorithm = FovAlgorithm::Basic; // rendering
const FOV_LIGHT_WALLS: bool = true; // rendering
const TORCH_RADIUS: i32 = 10; // rendering
const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 }; // rendering
const COLOR_LIGHT_WALL: Color = Color { r: 130, g: 110, b: 50}; // rendering
const COLOR_DARK_GROUND: Color = Color { r: 50, g: 50, b: 150 }; // rendering
const COLOR_LIGHT_GROUND: Color = Color { r: 200, g: 180, b: 50 }; // rendering
const BAR_WIDTH: i32 = 20; // rendering
const PANEL_HEIGHT: i32 = 7; // rendering
const SCREEN_WIDTH: i32 = 80; // rendering, menu
const SCREEN_HEIGHT: i32 = 50; // rendering, menu
const PANEL_Y: i32 = SCREEN_HEIGHT - PANEL_HEIGHT; // rendering
const MSG_X: i32 = BAR_WIDTH + 2; // rendering
const MSG_WIDTH: i32 = SCREEN_WIDTH - BAR_WIDTH -2; // rendering
const MSG_HEIGHT: usize = PANEL_HEIGHT as usize - 1; // rendering

const INVENTORY_WIDTH: i32 = 50; // menu

const MAP_WIDTH: i32 = 80; // mapping, rendering
const MAP_HEIGHT: i32 = 43; // mapping, rendering
const ROOM_MAX_SIZE: i32 = 10; // mapping
const ROOM_MIN_SIZE: i32 = 6; // mapping
const MAX_ROOMS: i32 = 30; // mapping

const LEVEL_UP_BASE: i32 = 200; // controls
const LEVEL_UP_FACTOR: i32 = 150; // controls
const CHARACTER_SCREEN_WIDTH: i32 = 30; // controls

const LEVEL_SCREEN_WIDTH: i32 = 40; // player_actions

const HEAL_AMOUNT: i32 = 40; // spellcasting
const LIGHTNING_RANGE: i32 = 5; // spellcasting
const LIGHTNING_DAMAGE: i32 = 40; // spellcasting
const CONFUSE_RANGE: i32 = 8; // spellcasting
const CONFUSE_NUM_TURNS: i32 = 10; // spellcasting
const FIREBALL_RADIUS: i32 = 3; // spellcasting
const FIREBALL_DAMAGE: i32 = 25; // spellcasting

// MAIN
fn main() {
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Mysterious Mysteries of the Abysslike Catacombs")
        .init();

    tcod::system::set_fps(LIMIT_FPS);

    let mut tcod = Tcod{
        root,
        con: Offscreen::new(MAP_WIDTH, MAP_HEIGHT),
        panel: Offscreen::new(SCREEN_WIDTH, PANEL_HEIGHT),
        fov: FovMap::new(MAP_WIDTH, MAP_HEIGHT),
        mouse: Default::default(),
    };

    app::menu::main_menu(&mut tcod);
}