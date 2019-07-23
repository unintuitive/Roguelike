use super::menu;
use super::data::{ Object, Game, Tcod, MessageLog, };
use crate::{PLAYER, LEVEL_UP_BASE, LEVEL_UP_FACTOR, LEVEL_SCREEN_WIDTH};

pub fn level_up(objects: &mut [Object], game: &mut Game, tcod: &mut Tcod) {
    let player = &mut objects[PLAYER];
    let level_up_xp = LEVEL_UP_BASE + player.level * LEVEL_UP_FACTOR;

    if player.fighter.as_ref().map_or(0, |f| f.xp) >= level_up_xp {
        player.level += 1;
        game.log.add(
            format!(
                "Your battle skills grow stronger! You reached level {}!",
                player.level
            ),
            tcod::colors::YELLOW,
        );

        let fighter = player.fighter.as_mut().unwrap();
        let mut choice = None;
        while choice.is_none() {
            choice = menu::menu(
                "Level up! Choose a stat to raise:\n",
                &[
                    format!("Constitution (+20 HP, from {}", fighter.base_max_hp),
                    format!("Strength (+1 attack, from {}", fighter.base_power),
                    format!("Agility (+1 defense, from {}", fighter.base_defense),
                ],
                LEVEL_SCREEN_WIDTH,
                &mut tcod.root,
            );
        }
        fighter.xp -= level_up_xp;
        match choice.unwrap() {
            0 => {
                fighter.base_max_hp += 20;
                fighter.hp += 20;
            }
            1 => { fighter.base_power += 1 }
            2 => { fighter.base_defense += 1 }
            _ => unreachable!(),
        }
    }
}
