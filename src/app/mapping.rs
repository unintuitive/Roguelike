use std::cmp;
use rand::Rng;
use rand::distributions::{Weighted, WeightedChoice, IndependentSample};
use super::data::{
    Object,
    Tile,
    Map,
    Rect,
    Transition,
    Fighter,
    DeathCallback,
    Ai,
    Item,
    Game,
    Tcod,
    MessageLog
};
use super::util;
use crate::{PLAYER, MAP_HEIGHT, MAP_WIDTH, ROOM_MAX_SIZE, ROOM_MIN_SIZE, MAX_ROOMS};
use super::rendering;
use crate::app::equipment::{Equipment, Slot};

// TODO: Refactor out map to game
pub fn create_room(room: Rect, map: &mut Map) {
    for x in (room.x1 + 1)..room.x2 {
        for y in (room.y1 + 1)..room.y2 {
            map[x as usize][y as usize] = Tile::empty();
        }
    }
}

// TODO: Refactor out map to game
pub fn create_h_tunnel(x1: i32, x2: i32, y: i32, map: &mut Map) {
    for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

// TODO: Refactor out map to game
pub fn create_v_tunnel(y1: i32, y2: i32, x: i32, map: &mut Map) {
    for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
        map[x as usize][y as usize] = Tile::empty();
    }
}

// TODO: Refactor out map to game
pub fn is_blocked(x: i32, y: i32, map: &Map, objects: &[Object]) -> bool {
    if map[x as usize][y as usize].blocked {
        return true;
    }

    objects.iter().any(|object| {
        object.blocks && object.pos() == (x, y)
    })
}

// TODO: Refactor out map to game
pub fn place_objects(room: Rect, map: &Map, objects: &mut Vec<Object>, level: u32) {
    let max_monsters = util::from_dungeon_level(
        &[
            Transition { level: 1, value: 2 },
            Transition { level: 4, value: 3 },
            Transition { level: 6, value: 5 },
        ],
        level,
    );

    let num_monsters = rand::thread_rng().gen_range(0, max_monsters + 1);

    let troll_chance = util::from_dungeon_level(
        &[
            Transition {
                level: 2,
                value: 5,
            },
            Transition {
                level: 3,
                value: 15,
            },
            Transition {
                level: 5,
                value: 30,
            },
            Transition {
                level: 7,
                value: 60,
            },
        ],
        level,
    );

    let draco_chance = util::from_dungeon_level(
        &[
            Transition {
                level: 1,
                value: 5,
            },
            Transition {
                level: 3,
                value: 20,
            },
            Transition {
                level: 5,
                value: 60,
            },
            Transition {
                level: 7,
                value: 80,
            },
        ],
        level,
    );

    // A random table of monsters to potentially create
    let monster_chances = &mut [
        Weighted {
            weight: 80,
            item: "orc",
        },
        Weighted {
            weight: troll_chance,
            item: "troll",
        },
        Weighted {
            weight: draco_chance,
            item: "draco",
        },
    ];
    let monster_choice = WeightedChoice::new(monster_chances);

    for _ in 0..num_monsters {
        let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);

        if !is_blocked(x, y, map, objects) {
            let mut monster = match monster_choice.ind_sample(&mut rand::thread_rng()) {
                "orc" => {
                    let mut orc = Object::new(x, y, 'o', tcod::colors::DESATURATED_GREEN, "orc", true);
                    orc.fighter = Some(Fighter {
                        base_max_hp: 20,
                        hp: 20,
                        base_power: 4,
                        base_defense: 0,
                        on_death: DeathCallback::Monster,
                        xp: 35,
                    });
                    orc.ai = Some(Ai::Basic);
                    orc
                }
                "troll" => {
                    let mut troll = Object::new(x, y, 'T', tcod::colors::DARKER_GREEN, "troll", true);
                    troll.fighter = Some(Fighter {
                        base_max_hp: 30,
                        hp: 16,
                        base_power: 8,
                        base_defense: 2,
                        on_death: DeathCallback::Monster,
                        xp: 100,
                    });
                    troll.ai = Some(Ai::Basic);
                    troll
                }
                "draco" => {
                    let mut draco = Object::new(x, y, 'd', tcod::colors::DARK_BLUE, "draco", true);
                    draco.fighter = Some(Fighter {
                        base_max_hp: 25,
                        hp: 25,
                        base_power: 5,
                        base_defense: 1,
                        on_death: DeathCallback::Monster,
                        xp: 65,
                    });
                    draco.ai = Some(Ai::Basic);
                    draco
                }
                _ => unreachable!(),
            };
            monster.alive = true;
            objects.push(monster);
        }
    }

    let max_items = util::from_dungeon_level(
        &[
            Transition { level: 1, value: 1 },
            Transition { level: 4, value: 2 },
        ],
        level,
    );

    // A random table of items to create
    let item_chances = &mut [
        Weighted {
            weight: 35,
            item: Item::Heal,
        },
        Weighted {
            weight: util::from_dungeon_level(
                &[Transition {
                    level: 4,
                    value: 25,
                }],
                level
            ),
            item: Item::Lightning,
        },
        Weighted {
            weight: util::from_dungeon_level(
                &[Transition {
                    level: 4,
                    value: 25,
                }],
                level
            ),
            item: Item::Fireball,
        },
        Weighted {
            weight: util::from_dungeon_level(
                &[Transition {
                    level: 2,
                    value: 10,
                }],
                level,
            ),
            item: Item::Confuse,
        },
        Weighted {
            weight: util::from_dungeon_level(
                &[Transition {
                    level: 4,
                    value: 5
                }],
                level
            ),
            item: Item::Sword },
        Weighted {
            weight: util::from_dungeon_level(
                &[Transition {
                    level: 8,
                    value: 15,
                }],
                level
            ),
            item: Item::Shield,
        }
    ];

    let num_items = rand::thread_rng().gen_range(0, max_items + 1);
    let item_choice = WeightedChoice::new(item_chances);

    for _ in 0..num_items {
        let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);

        if !is_blocked(x, y, map, objects) {
//            let dice = rand::random::<f32>();
            let mut item = match item_choice.ind_sample(&mut rand::thread_rng()) {
                Item::Heal => {
                    let mut object = Object::new(x, y, '!', tcod::colors::VIOLET, "healing potion", false);
                    object.item = Some(Item::Heal);
                    object
                }
                Item::Lightning => {
                    let mut object = Object::new(
                        x,
                        y,
                        '#',
                        tcod::colors::LIGHT_YELLOW,
                        "scroll of lightning bolt",
                        false,
                    );
                    object.item = Some(Item::Lightning);
                    object
                }
                Item::Fireball => {
                    let mut object =
                        Object::new(x, y, '#', tcod::colors::ORANGE, "scroll of fireball", false);
                    object.item = Some(Item::Fireball);
                    object
                }
                Item::Confuse => {
                    let mut object = Object::new(
                        x,
                        y,
                        '#',
                        tcod::colors::LIGHT_AZURE,
                        "scroll of confusion",
                        false
                    );
                    object.item = Some(Item::Confuse);
                    object
                }
                Item::Sword => {
                    let mut object = Object::new(x, y, '/',tcod::colors::SKY,"sword",false);
                    object.item = Some(Item::Sword);
                    object.equipment = Some(Equipment{
                        equipped: false,
                        slot: Slot::RightHand,
                        power_bonus: 3,
                        defense_bonus: 0,
                        max_hp_bonus: 0,
                    });
                    object
                }
                Item::Shield => {
                    let mut object = Object::new(x, y, '[', tcod::colors::DARKER_ORANGE, "shield", false);
                    object.item = Some(Item::Shield);
                    object.equipment = Some(Equipment {
                        equipped: false,
                        slot: Slot::LeftHand,
                        power_bonus: 0,
                        defense_bonus: 1,
                        max_hp_bonus: 0,
                    });
                    object
                }
            };
            item.always_visible = true;
            objects.push(item);
        }
    }
}

// Make Map
pub fn make_map(objects: &mut Vec<Object>, level: u32) -> Map {
    let mut map = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];
    assert_eq!(&objects[PLAYER] as *const _, &objects[0] as *const _);
    objects.truncate(1);

    let mut rooms= vec![];

    for _ in 0..MAX_ROOMS {
        // Random room width and height
        let w = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);
        let h = rand::thread_rng().gen_range(ROOM_MIN_SIZE, ROOM_MAX_SIZE + 1);

        // Random room position within map boundaries
        let x = rand::thread_rng().gen_range(0, MAP_WIDTH - w);
        let y = rand::thread_rng().gen_range(0, MAP_HEIGHT - h);

        let new_room = Rect::new(x, y, w, h);

        let failed = rooms
            .iter()
            .any(|other_room| new_room.intersects_with(other_room));

        if !failed {
            create_room(new_room, &mut map);
            place_objects(new_room, &map, objects, level);

            let (new_x, new_y) = new_room.center();

            if rooms.is_empty() {
                objects[PLAYER].set_pos(new_x, new_y);
            } else {
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

                if rand::random() {
                    create_h_tunnel(prev_x, new_x, prev_y, &mut map);
                    create_v_tunnel(prev_y, new_y, new_x, &mut map);
                } else {
                    create_v_tunnel(prev_y, new_y, prev_x, &mut map);
                    create_h_tunnel(prev_x, new_x, new_y, &mut map);
                }
            }
            rooms.push(new_room);
        }

    }

    // Create stairs at the center of the last room generated.
    let (last_room_x, last_room_y) = rooms[rooms.len() - 1].center();
    let mut stairs = Object::new(
        last_room_x,
        last_room_y,
        '<',
        tcod::colors::WHITE,
        "stairs",
        false,
    );
    stairs.always_visible = true;
    objects.push(stairs);
    map
}

pub fn next_level(objects: &mut Vec<Object>, game: &mut Game, tcod: &mut Tcod) {
    game.log.add(
        "You take a moment to rst, and recover your strength.",
        tcod::colors::VIOLET,
    );
    let heal_hp = objects[PLAYER].max_hp(game) / 2;
    objects[PLAYER].heal(heal_hp, game);

    game.log.add(
        "After a rare moment of peace, you descend deeper into the catacombs...",
        tcod::colors::RED,
    );
    game.dungeon_level += 1;
    game.map = make_map(objects, game.dungeon_level);
    rendering::init_fov(&game.map, tcod);
}
