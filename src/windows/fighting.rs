use crate::{Game, State};
use rand::Rng;

pub fn fighting_window(game: &mut Game) {
    println!("running window + {:#?} + fighting", game.state.get());
    match rand::thread_rng().gen_range(0..2) {
        0 => game.state.push(State::Dead),
        1 => {
            game.state.pop();
        }
        _ => panic!("out of range random number"),
    };
    game.state.push(State::Transition);
}
