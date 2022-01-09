use crate::Game;

pub fn transition_window(game: &mut Game) {
    println!("... Transition ...");
    game.state.pop();
}
