mod dead;
mod exploring;
mod fighting;
mod menu;
mod transition;

pub use dead::dead_window;
pub use exploring::exploring_window;
pub use fighting::fighting_window;
pub use menu::menu_window;
pub use transition::transition_window;

use crate::Game;

pub type Window = dyn Fn(&mut Game);
