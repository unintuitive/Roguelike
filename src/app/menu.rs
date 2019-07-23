use tcod::console::*;

use super::meta;
use super::rendering;
use super::data::{
    Tcod,
    Object,
};
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH, INVENTORY_WIDTH};

pub fn menu<T: AsRef<str>>(
    header: &str,
    options: &[T],
    width: i32,
    root: &mut Root,
) -> Option<usize> {

    assert!(
        options.len() <= 26,
        "Cannot have a menu with more than 26 options."
    );

    let header_height= if header.is_empty() {
        0
    } else {
        root.get_height_rect(0, 0, width, SCREEN_HEIGHT, header)
    };

    let height = options.len() as i32 + header_height;

    let mut window = Offscreen::new(width, height);

    window.set_default_foreground(tcod::colors::WHITE);
    window.print_rect_ex(
        0,
        0,
        width,
        height,
        BackgroundFlag::None,
        TextAlignment::Left,
        header,
    );

    for (index, option_text) in options.iter().enumerate() {
        let menu_letter = (b'a' + index as u8) as char;
        let text = format!("({}) {}", menu_letter, option_text.as_ref());
        window.print_ex(
            0,
            header_height + index as i32,
            BackgroundFlag::None,
            TextAlignment::Left,
            text,
        );
    }

    let x = SCREEN_WIDTH / 2 - width / 2;
    let y = SCREEN_HEIGHT / 2 - height /2;
    tcod::console::blit(
        &mut window,
        (0, 0),
        (width, height),
        root,
        (x, y),
        1.0,
        0.7,
    );

    root.flush();
    let key = root.wait_for_keypress(true);

    if key.printable.is_alphabetic() {
        let index = key.printable.to_ascii_lowercase() as usize - 'a' as usize;
        if index < options.len() {
            Some(index)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn msgbox(text: &str, width: i32, root: &mut Root) {
    let options: &[&str] = &[];
    menu(text, options, width, root);
}

pub fn main_menu(tcod: &mut Tcod) {
    let img = tcod::image::Image::from_file("menu_background.png")
        .ok()
        .expect("Background image not found.");

    while !tcod.root.window_closed() {
        tcod::image::blit_2x(&img, (0, 0), (-1, -1), &mut tcod.root, (0, 0));

        tcod.root.set_default_foreground(tcod::colors::LIGHT_YELLOW);
        tcod.root.print_ex(
            SCREEN_WIDTH / 2,
            SCREEN_HEIGHT / 2 - 4,
            BackgroundFlag::None,
            TextAlignment::Center,
            "MYSTERIOUS MYSTERIES OF THE ABYSSLIKE CATACOMBS"
        );

        tcod.root.print_ex(
            SCREEN_WIDTH / 2,
            SCREEN_HEIGHT - 2,
            BackgroundFlag::None,
            TextAlignment::Center,
            "By Eric",
        );

        let choices = &["Play a new game", "Continue last game", "Quit"];
        let choice = menu("", choices, 24, &mut tcod.root);

        match choice {
            // Play a new game
            Some(0) => {
                let (mut objects, mut game) = meta::new_game(tcod);
                meta::play_game(&mut objects, &mut game, tcod);
            }
            // Load saved game
            Some(1) => {
//                let (mut objects, mut game) = load_game().unwrap();
                match meta::load_game() {
                    Ok((mut objects, mut game)) => {
                        rendering::init_fov(&game.map, tcod);
                        meta::play_game(&mut objects, &mut game, tcod);
                    }
                    Err(_e) => {
                        msgbox("\nNo saved game to load.\n", 24, &mut tcod.root);
                        continue;
                    }
                }
            }
            // Exit (auto-saves)
            Some(2) => {
                break;
            }
            _ => {}
        }
    }
}

pub fn inventory_menu(
    inventory: &[Object],
    header: &str,
    root: &mut Root,
) -> Option<usize> {
    let options = if inventory.len() == 0 {
        vec!["Inventory is empty.".into()]
    } else {
//        inventory.iter().map(|item| { item.name.clone()}).collect()
        inventory
            .iter()
            .map(|item| {
                match item.equipment {
                    Some(equipment) if equipment.equipped => {
                        format!("{} (on {})", item.name, equipment.slot)
                    }
                    _ => item.name.clone()
                }
            })
            .collect()
    };

    let inventory_index = menu(header, &options, INVENTORY_WIDTH, root);

    if inventory.len() > 0 {
        inventory_index
    } else {
        None
    }
}
