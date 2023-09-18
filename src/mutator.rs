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
    for i in 0..state.event_queue.len() {
        match state.event_queue[i] {
            Action::MoveRightA => {
                state.move_right();
                state.last_move_right_activation = Some(Instant::now());
            }
            Action::MoveLeftA => {
                state.move_left();
                state.last_move_left_activation = Some(Instant::now());
            }
            _ => (),
        }
    }
}