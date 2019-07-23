use tcod::input::{ Key, Mouse};
use tcod::map::{ Map as FovMap};

use super::data::{ Object, Game, Tcod, PlayerAction, MessageLog };
use super::movement;
use super::items;
use super::menu;
use super::mapping;
use crate::{PLAYER, LEVEL_UP_BASE, LEVEL_UP_FACTOR, CHARACTER_SCREEN_WIDTH};

pub fn handle_keys(
    key: Key,
    objects: &mut Vec<Object>,
    game: &mut Game,
    tcod: &mut Tcod,
) -> PlayerAction {
    use tcod::input::KeyCode::*;
    use PlayerAction::*;

    //let key = root.wait_for_keypress(true);
    let player_alive = objects[PLAYER].alive;
    match (key, player_alive) {
        // Movement Keybindings
        (Key { code: Text, .. }, true) => {
            // This is an unfortunate workaround. If I try to use "printable"
            // instead, I get duplicated movement -- the user moves two steps
            // for every keypress. Matching on the key.text() instead works.
            match key.text() {
                "w" => {
                    movement::player_move_or_attack(0, -1, objects, game);
                    TookTurn
                }
                "x" => {
                    movement::player_move_or_attack(0, 1, objects, game);
                    TookTurn
                }
                "a" => {
                    movement::player_move_or_attack(-1, 0, objects, game);
                    TookTurn
                }
                "d" => {
                    movement::player_move_or_attack(1, 0, objects, game);
                    TookTurn
                }
                "q" => {
                    movement::player_move_or_attack(-1, -1, objects, game);
                    TookTurn
                }
                "e" => {
                    movement::player_move_or_attack(1, -1, objects, game);
                    TookTurn
                }
                "z" => {
                    movement::player_move_or_attack(-1, 1, objects, game);
                    TookTurn
                }
                "c" => {
                    movement::player_move_or_attack(1, 1, objects, game);
                    TookTurn
                }
                // Either go down stairs, or just wait.
                // TODO: This should probably be something different.
                "s" => {
                    let player_on_stairs = objects
                        .iter()
                        .any(|object| object.pos() == objects[PLAYER].pos() && object.name == "stairs");
                    if player_on_stairs {
                        game.log.add(
                            "You cant do that on television!",
                            tcod::colors::DARK_ORANGE,
                        );
                        mapping::next_level(objects, game, tcod);
                        DidntTakeTurn
                    } else {
                        TookTurn
                    }
                }
                _ => DidntTakeTurn
            }
        }
        (Key { code: Up, .. }, true) => {
            movement::player_move_or_attack(0, -1, objects, game);
            TookTurn
        }
        (Key { code: Down, .. }, true) => {
            movement::player_move_or_attack(0, 1, objects, game);
            TookTurn
        }
        (Key { code: Left, .. }, true) => {
            movement::player_move_or_attack(-1, 0, objects, game);
            TookTurn
        }
        (Key { code: Right, .. }, true) => {
            movement::player_move_or_attack(1, 0, objects, game);
            TookTurn
        }
        // Grab item
        (Key { printable: 'g', .. }, true) => {
            let item_id = objects
                .iter()
                .position(|object| object.pos() == objects[PLAYER].pos() && object.item.is_some());
            if let Some(item_id) = item_id {
                items::pick_item_up(item_id, objects, game);
            }
            DidntTakeTurn
        }
        // Drop item
        (Key { printable: 'o', .. }, true) => {
            let inventory_index = menu::inventory_menu(
                &mut game.inventory,
                "Press the key next to an item to drop it, or any other to cancel.\n",
                &mut tcod.root,
            );
            if let Some(inventory_index) = inventory_index {
                items::drop_item(inventory_index, objects, game);
            }
            DidntTakeTurn
        }
        // Show inventory
        (Key { printable: 'i', ..}, true) => {
            let inventory_index = menu::inventory_menu(
                &mut game.inventory,
                "Press the key next to an item to use it, or any other to cancel.\n",
                &mut tcod.root
            );
            if let Some(inventory_index) = inventory_index {
                items::use_item(inventory_index, objects, game, tcod );
            }
            DidntTakeTurn
        }
        // TODO: < doesn't work. It's a bug, it seems.
        (Key { printable: '<', .. }, true) => {
            let player_on_stairs = objects
                .iter()
                .any(|object| object.pos() == objects[PLAYER].pos() && object.name == "stairs");
            if player_on_stairs {
                game.log.add(
                    "You cant do that on television!",
                    tcod::colors::DARK_ORANGE,
                );
                mapping::next_level(objects, game, tcod);
            }
            DidntTakeTurn
        }
        (Key { printable: 'u', .. }, true) => {
            let player = &objects[PLAYER];
            let level = player.level;
            let level_up_xp = LEVEL_UP_BASE + player.level * LEVEL_UP_FACTOR;
            if let Some(fighter) = player.fighter.as_ref() {
                let msg = format!(
                    "Character information:
\n
Level: {}\n
Experience: {}\n
XP to level up: {}\n
\n
Maximum HP: {}\n
Attack: {}\n
Defense: {}",
                    level, fighter.xp, level_up_xp, fighter.base_max_hp, player.power(game), fighter.base_defense
                );
                menu::msgbox(&msg, CHARACTER_SCREEN_WIDTH, &mut tcod.root)
            }
            DidntTakeTurn
        }
        // Toggle fullscreen
        (
            Key {
                code: Enter,
                alt: true,
                ..
            },
            _,
        ) => {
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
            DidntTakeTurn
        }
        (Key { code: Escape, .. }, _) => Exit, // Exit the game
        _ => DidntTakeTurn,
    }
}

pub fn get_names_under_mouse(mouse: Mouse, objects: &[Object], fov_map: &FovMap) -> String {
    let (x, y) = (mouse.cx as i32, mouse.cy as i32);

    let names = objects
        .iter()
        .filter(|obj| { obj.pos() == (x, y) && fov_map.is_in_fov(obj.x, obj.y)})
        .map(|obj| obj.name.clone())
        .collect::<Vec<_>>();

    names.join(", ")
}
