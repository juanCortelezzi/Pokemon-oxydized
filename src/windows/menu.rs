use crate::{Game, State};
use rand::Rng;

pub fn menu_window(game: &mut Game) {
    println!("running window + {:#?} + menu", game.state.get());
    match rand::thread_rng().gen_range(0..2) {
        0 => game.state.push(State::Exploring),
        1 => game.state.push(State::Quit),
        _ => panic!("out of range random number"),
    };
}
