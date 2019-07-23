use std::cmp;
use super::data::{ Transition, Object, Game, MessageLog, PlayerAction };
use crate::PLAYER;

pub fn mut_two<T>(first_index: usize, second_index: usize, items: &mut [T]) -> (&mut T, &mut T) {
    assert_ne!(first_index, second_index);
    let split_at_index = cmp::max(first_index, second_index);
    let(first_slice, second_slice) = items.split_at_mut(split_at_index);
    if first_index < second_index {
        (&mut first_slice[first_index], &mut second_slice[0])
    } else {
        (&mut second_slice[0], &mut first_slice[second_index])
    }
}

pub fn from_dungeon_level(table: &[Transition], level: u32) -> u32 {
    table
        .iter()
        .rev()
        .find(|transition| level >= transition.level)
        .map_or(0, |transition| transition.value)
}

// DEATH
pub fn player_death(player: &mut Object, game: &mut Game) {
    game.log.add("You died!", tcod::colors::DARK_RED);
    player.char = '%';
    player.color = tcod::colors::DARK_RED;
    player.alive = false;
    player.fighter = None;
    player.name = format!("The remains of {}. R.I.P.", player.name);
}

pub fn monster_death(monster: &mut Object, game: &mut Game) {
    game.log.add(
        format!(
            "{} dies in agony! You gain {} XP.",
            monster.name, monster.fighter.unwrap().xp,
        ),
        tcod::colors::DARK_ORANGE,
    );
    monster.char = '%';
    monster.color = tcod::colors::DARK_RED;
    monster.blocks = false;
    monster.fighter = None;
    monster.ai = None;
    monster.alive = false;
    monster.name = format!("The stinking remains of a {}", monster.name);
}

pub fn growl(objects: &mut Vec<Object>, game: &mut Game) {
    if objects[PLAYER].alive {
        game.log.add(
            format!("{} power is {} and he growls.", objects[PLAYER].name, objects[PLAYER].power(game)),
            tcod::colors::WHITE
        );
    }
}
//
//pub fn growl(objects: &mut Vec<Object>, game: &mut Game) {
////    if objects[PLAYER].alive && objects[PLAYER].  != PlayerAction::DidntTakeTurn {
//    if objects[PLAYER].alive {
//        for object in objects {
//            if object.name == "Player".to_string() {
//                if object.alive {
//                    game.log.add(
//                        format!("Player power is {} and he growls.", object.power(game)),
//                        tcod::colors::WHITE
//                    );
//                }
//            }
//        }
//    }
//}
