use super::targeting;
use super::data::{ Object, Game, Tcod, MessageLog, UseResult, Ai };
use crate::{PLAYER, HEAL_AMOUNT, LIGHTNING_RANGE, LIGHTNING_DAMAGE, CONFUSE_RANGE, CONFUSE_NUM_TURNS, FIREBALL_RADIUS, FIREBALL_DAMAGE};

pub fn cast_heal(
    _inventory_id: usize,
    objects: &mut [Object],
    game: &mut Game,
    _tcod: &mut Tcod,
) -> UseResult {
    let player = &mut objects[PLAYER];
    if let Some(fighter) = player.fighter {
        if fighter.hp == player.max_hp(game) {
            game.log.add("You are already at full health.", tcod::colors::RED);
            return UseResult::Cancelled;
        }
        game.log.add(
            "Your wounds start to feel better!",
            tcod::colors::LIGHT_VIOLET,
        );
        player.heal(HEAL_AMOUNT, game);
        return UseResult::UsedUp;
    }
    UseResult::Cancelled
}

pub fn cast_lightning(
    _inventory_id: usize,
    objects: &mut [Object],
    game: &mut Game,
    tcod: &mut Tcod,
) -> UseResult {
    let monster_id = targeting::closest_monster(LIGHTNING_RANGE, objects, tcod);
    if let Some(monster_id) = monster_id {
        game.log.add(
            format!(
                "A lightning bolt strikes the {} with a deafening thunderclap! It damages for {} points.",
                objects[monster_id].name, LIGHTNING_DAMAGE
            ),
            tcod::colors::LIGHT_BLUE,
        );
        if let Some(xp) = objects[monster_id].take_damage(LIGHTNING_DAMAGE, game) {
            objects[PLAYER].fighter.as_mut().unwrap().xp += xp;
        }
        UseResult::UsedUp
    } else {
        game.log.add("No enemy is close enough to strike.", tcod::colors::RED);
        UseResult::Cancelled
    }
}

pub fn cast_confuse(
    _inventory_id: usize,
    objects: &mut [Object],
    game: &mut Game,
    tcod: &mut Tcod,
) -> UseResult {
//    let monster_id = closest_monster(CONFUSE_RANGE, objects, tcod);
    game.log.add(
        "Left-click a target tile for the faireball, or right-click to cancel.",
        tcod::colors::LIGHT_CYAN,
    );
    let monster_id = targeting::target_monster(Some(CONFUSE_RANGE as f32), objects, game, tcod);
    if let Some(monster_id) = monster_id {
        let old_ai = objects[monster_id].ai.take().unwrap_or(Ai::Basic);
        objects[monster_id].ai = Some(Ai::Confused {
            previous_ai: Box::new(old_ai),
            num_turns: CONFUSE_NUM_TURNS,
        });
        game.log.add(
            format!(
                "The eyes of {} look vacant, and he starts to stumble around!",
                objects[monster_id].name
            ),
            tcod::colors::LIGHT_GREEN,
        );
        UseResult::UsedUp
    } else {
        game.log.add("No enemy is close enough to strike.", tcod::colors::RED);
        UseResult::Cancelled
    }
}

pub fn cast_fireball(
    _inventory_id: usize,
    objects: &mut [Object],
    game: &mut Game,
    tcod: &mut Tcod,
) -> UseResult {
    game.log.add(
        "Left-click a target tile for the faireball, or right-click to cancel.",
        tcod::colors::LIGHT_CYAN,
    );
    let (x, y) = match targeting::target_tile(None, objects, game, tcod) {
        Some(tile_pos) => tile_pos,
        None => return UseResult::Cancelled,
    };
    game.log.add(
        format!(
            "The fireball explodes, burning everything within {} tiles!",
            FIREBALL_RADIUS
        ),
        tcod::colors::ORANGE,
    );

    let mut xp_to_gain = 0;
    for (id, obj) in objects.iter_mut().enumerate() {
        if obj.distance(x, y) < FIREBALL_RADIUS as f32 && obj.fighter.is_some() {
            game.log.add(
                format!(
                    "The {} is burned for {} points of damage.",
                    obj.name, FIREBALL_DAMAGE
                ),
                tcod::colors::ORANGE,
            );
            if let Some(xp) = obj.take_damage(FIREBALL_DAMAGE, game) {
                if id != PLAYER {
                    xp_to_gain += xp;
                }
            }
        }
    }
    // TODO something around here can cause a crash. Why?
    objects[PLAYER].fighter.as_mut().unwrap().xp += xp_to_gain;
    UseResult::UsedUp
}
