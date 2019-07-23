use tcod::map::{Map as FovMap};
use super::data::{ Object, Game, Ai, MessageLog };
use super::util;
use super::movement;
use crate::PLAYER;
use rand::Rng;

pub fn ai_take_turn(
    monster_id: usize,
    objects: &mut [Object],
    game: &mut Game,
    fov_map: &FovMap,
) {
    use Ai::*;
    if let Some(ai) = objects[monster_id].ai.take() {
        let new_ai = match ai {
            Basic => ai_basic(monster_id,  objects, game, fov_map),
            Confused {
                previous_ai,
                num_turns,
            } => ai_confused(monster_id,  previous_ai, num_turns,  objects, game),
        };
        objects[monster_id].ai = Some(new_ai);
    }
}

pub fn ai_basic(
    monster_id: usize,
    objects: &mut [Object],
    game: &mut Game,
    fov_map: &FovMap,
) -> Ai {
    let (monster_x, monster_y) = objects[monster_id].pos();
    if fov_map.is_in_fov(monster_x, monster_y) {
        if objects[monster_id].distance_to(&objects[PLAYER]) >= 2.0 {
            let (player_x, player_y) = objects[PLAYER].pos();
            movement::move_toward(monster_id, player_x, player_y, objects, game);
        } else if objects[PLAYER].fighter.map_or(false, |f| f.hp > 0) {
//            message(
//                messages,
//                format!("The {} attacks the player!", monster.name),
//                colors::WHITE
//            );
            let (monster, player) = util::mut_two(monster_id, PLAYER, objects);
            monster.attack(player, game);
        }
    }
    Ai::Basic
}

pub fn ai_confused(
    monster_id: usize,
    previous_ai: Box<Ai>,
    num_turns: i32,
    objects: &mut [Object],
    game: &mut Game,
) -> Ai {
    if num_turns >= 0 {
        // Confusion!
        // Move in a random direction, decreasing the num_turns
        game.log.add(
            format!("The {} stumbles around in a daze!", objects[monster_id].name),
            tcod::colors::RED,
        );
        movement::move_by(
            monster_id,
            rand::thread_rng().gen_range(-1, 2),
            rand::thread_rng().gen_range(-1, 2),
            objects,
            game,
        );
        Ai::Confused {
            previous_ai,
            num_turns: num_turns - 1,
        }
    } else {
        game.log.add(
            format!("The {} is no longer confused.", objects[monster_id].name),
            tcod::colors::RED,
        );
        *previous_ai
    }
}
