use crate::{Game, State};

pub fn dead_window(game: &mut Game) {
    println!("running window + {:#?} + dead", game.state.get());
    game.state.push(State::Reset);
}
