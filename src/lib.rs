pub mod windows;

#[derive(Debug)]
pub enum State {
    Menu,
    Fighting,
    Dead,
    Exploring,
    Transition,
    Reset,
    Quit,
}

#[derive(Debug)]
pub struct WindowState {
    state_stack: Vec<State>,
}

impl WindowState {
    const DEFAULT_STATE: State = State::Menu;

    pub fn new() -> Self {
        Self {
            state_stack: vec![Self::DEFAULT_STATE],
        }
    }

    /// Get a reference to the current state on the stack.
    pub fn get(&self) -> &State {
        self.state_stack.last().unwrap()
    }

    /// Push a State to the stack
    pub fn push(&mut self, state: State) {
        self.state_stack.push(state);
    }

    /// Pop a State from the stack
    pub fn pop(&mut self) -> State {
        // its ok to unwrap because the game would have been stopped if the stack was empty
        self.state_stack.pop().unwrap()
    }

    /// Returns true if the state stack is empty
    pub fn is_empty(&self) -> bool {
        self.state_stack.is_empty()
    }

    /// Resets the stack to its default state
    fn reset(&mut self) {
        self.state_stack.clear();
        self.state_stack.push(Self::DEFAULT_STATE)
    }

    /// Clears the stack
    fn clear(&mut self) {
        self.state_stack.clear();
    }
}

impl Default for WindowState {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for WindowState {
    fn drop(&mut self) {
        println!("{:#?}", self.state_stack);
    }
}

#[derive(Debug)]
pub struct Game {
    pub state: WindowState,
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: Default::default(),
        }
    }

    // run_loop is the game loop that executes the respective handler funtion of each state.
    pub fn run_loop(&mut self) {
        while !self.state.is_empty() {
            match self.state.get() {
                State::Menu => windows::menu_window(self),
                State::Fighting => windows::fighting_window(self),
                State::Dead => windows::dead_window(self),
                State::Exploring => windows::exploring_window(self),
                State::Transition => windows::transition_window(self),
                State::Reset => self.state.reset(),
                // State::Quit => self.state.clear(),
                State::Quit => break,
            }
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
