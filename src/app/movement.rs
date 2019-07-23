use super::mapping;
use super::util;
use super::data::{ Object, Game, };
use crate::PLAYER;

pub fn move_by(id: usize, dx: i32, dy: i32, objects: &mut [Object], game: &mut Game) {
    let (x, y) = objects[id].pos();
    if !mapping::is_blocked(x + dx, y + dy, &game.map, objects) {
        objects[id] .set_pos(x + dx, y + dy);
    }
}

pub fn move_toward(id: usize, target_x: i32, target_y: i32, objects: &mut [Object], game: &mut Game ) {
    let dx = target_x - objects[id].x;
    let dy = target_y - objects[id].y;
    let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();

    let dx = (dx as f32 / distance).round() as i32;
    let dy = (dy as f32 / distance).round() as i32;
    move_by(id, dx, dy, objects, game);
}

pub fn player_move_or_attack(dx: i32, dy: i32, objects: &mut [Object], game: &mut Game) {
    let x = objects[PLAYER].x + dx;
    let y = objects[PLAYER].y + dy;

    let target_id = objects
        .iter()
        .position(|object| object.fighter.is_some() && object.pos() == (x, y));

    match target_id {
        Some(target_id) => {
//            message(
//                messages,
//                format!("The player attacks the {}!", objects[target_id].name),
//                colors::WHITE);
            let (player, target) = util::mut_two(PLAYER, target_id, objects);
            player.attack(target, game);
        }
        None => {
            move_by(PLAYER, dx, dy, objects, game);
        }
    }
}
