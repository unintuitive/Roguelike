use super::data::{ Object, Game, Tcod, UseResult, MessageLog, Item };
use super::spellcasting;
use super::equipment;
use crate::PLAYER;
use crate::app::equipment::get_equipped_in_slot;

pub fn pick_item_up(
    object_id: usize,
    objects: &mut Vec<Object>,
    game: &mut Game,
) {
    if game.inventory.len() >= 26 {
        game.log.add(
            format!(
                "Your inventory is full, cannot pick up {}.",
                objects[object_id].name
            ),
            tcod::colors::RED,
        );
    } else {
        let item = objects.swap_remove(object_id);
        game.log.add(
            format!("You picked up a {}!", item.name),
            tcod::colors::GREEN,
        );
        let index = game.inventory.len();
        let slot = item.equipment.map(|e| e.slot);
        game.inventory.push(item);

        if let Some(slot) = slot {
            if get_equipped_in_slot(slot, &game.inventory).is_none() {
                game.inventory[index].equip(&mut game.log);
            }
        }
    }
}

pub fn drop_item(
    inventory_id: usize,
    objects: &mut Vec<Object>,
    game: &mut Game,
) {
    let mut item = game.inventory.remove(inventory_id);
    if item.equipment.is_some() {
        item.dequip(&mut game.log);
    }
    item.set_pos(objects[PLAYER].x, objects[PLAYER].y);
    game.log.add(
        format!("You dropped a {}.", item.name),
        tcod::colors::YELLOW,
    );
    objects.push(item);
}

pub fn use_item(
    inventory_id: usize,
    objects: &mut [Object],
    game: &mut Game,
    tcod: &mut Tcod,
) {
//    use data::Item::*;
    if let Some(item) = game.inventory[inventory_id].item {
        let on_use = match item {
            Item::Heal => spellcasting::cast_heal,
            Item::Lightning => spellcasting::cast_lightning,
            Item::Confuse => spellcasting::cast_confuse,
            Item::Fireball => spellcasting::cast_fireball,
            Item::Sword => equipment::toggle_equipment,
            Item::Shield => equipment::toggle_equipment,
        };

        match on_use(inventory_id, objects, game, tcod) {
            UseResult::UsedUp => {
                game.inventory.remove(inventory_id);
            }
            UseResult::UsedAndKept => {}, // Do nothing
            UseResult::Cancelled => {
                game.log.add("Cancelled", tcod::colors::WHITE);
            }
        }
    } else {
        game.log.add(
            format!("The {} cannot be used.", game.inventory[inventory_id].name),
            tcod::colors::WHITE,
        );
    }
}
