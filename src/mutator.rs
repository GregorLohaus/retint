use crate::gamestate::{Action, State};
use std::{
    error::Error,
    io::stdout,
    time::{Duration, Instant},
};

pub fn mutate(state: &mut State, elapsed: Duration) {
    // return state;
    // state.scalex = 3;
    state.spawn_tetromino();
    state.tetromino_to_board();
    for i in 0..state.eventqueue.len() {
        match state.eventqueue[i] {
            Action::MoveRightA => {
                state.move_right();
                state.lastMRA = Some(Instant::now());
            }
            Action::MoveLeftA => {
                state.move_left();
                state.lastMLA = Some(Instant::now());
            }
            _ => (),
        }
    }
}
