use super::util;
use super::mapping;
use super::controls;
use super::rendering;
use super::ai;
use super::player_actions;
use super::equipment::{ Equipment, Slot };
use super::data::{
    Object,
    Fighter,
    DeathCallback,
    Game,
    Tcod,
    MessageLog,
    PlayerAction,
    Item,
};
use tcod::input::{self, Event,};
use tcod::console::*;
use crate::PLAYER;

use std::io::{Read, Write};
use std::fs::File;
use std::error::Error;

pub fn new_game(tcod: &mut Tcod) -> (Vec<Object>, Game) {
    let mut player = Object::new(0, 0, '@', tcod::colors::WHITE, "Player", true);
    player.alive = true;
    player.fighter = Some(Fighter {
        base_max_hp: 100,
        hp: 100,
        base_defense: 1,
        base_power: 4,
        on_death: DeathCallback::Player,
        xp: 0,
    });

    let mut objects = vec![player];
    let level = 1;

    let mut game = Game {
        map: mapping::make_map(&mut objects, level), // Generate the map
        log: vec![], // Game messages log
        inventory: vec![], // Player inventory
        dungeon_level: 1,
    };

    // Give the player a dagger to start with
    let mut dagger = Object::new(0, 0, '-', tcod::colors::SKY, "dagger", false);
    dagger.item = Some(Item::Sword);
    dagger.equipment = Some(Equipment {
        equipped: true,
        slot: Slot::LeftHand,
        defense_bonus: 0,
        power_bonus: 2,
        max_hp_bonus: 0,
    });
    game.inventory.push(dagger);

    rendering::init_fov(&game.map, tcod);

    // Friendly welcoming message
    game.log.add(
        "Welcome, stranger, to the Mysterious Mysteries of the Abysslike Catacombs! Prepare to die.",
        tcod::colors::RED,
    );

    (objects, game)
}

// GAME LOOP
pub fn play_game(objects: &mut Vec<Object>, game: &mut Game, tcod: &mut Tcod) {
    let mut previous_player_position = (-1, -1);
    let mut key = Default::default();

    while !tcod.root.window_closed() {
        tcod.con.clear();

        match tcod::input::check_for_event(input::MOUSE | input::KEY_PRESS) {
            Some(
                (_, Event::Mouse(m))
            ) => tcod.mouse = m,
            Some(
                (_, Event::Key(k))
            ) => key = k,
            _ => key = Default::default()
        }

        let fov_recompute = previous_player_position != (objects[PLAYER].pos());
        rendering::render_all(
            fov_recompute,
            &objects,
            game,
            tcod,
        );

        tcod.root.flush();

        player_actions::level_up(objects, game, tcod);

        previous_player_position = objects[PLAYER].pos();
        let player_action = controls::handle_keys(
            key,
            objects,
            game,
            tcod,
        );
        if player_action == PlayerAction::Exit {
            save_game(objects, game).unwrap();
            break;
        }

        if objects[PLAYER].alive && player_action != PlayerAction::DidntTakeTurn {
//            util::growl(objects, game); // Useful for in-game debug
            for id in 0..objects.len() {
                if objects[id].ai.is_some() {
                    ai::ai_take_turn(id, objects, game, &tcod.fov);
                }
            }
        }
    }
}

pub fn save_game(objects: &[Object], game: &Game) -> Result<(), Box<Error>> {
    let save_data = serde_json::to_string(&(objects, game))?;
    let mut file = File::create("savegame")?;
    file.write_all(save_data.as_bytes())?;
    Ok(())
}

pub fn load_game() -> Result<(Vec<Object>, Game), Box<Error>> {
    let mut json_save_state = String::new();
    let mut file = File::open("savegame")?;
    file.read_to_string(&mut json_save_state)?;
    let result =
        serde_json::from_str::<(Vec<Object>, Game)>(&json_save_state)?;
    Ok(result)
}
