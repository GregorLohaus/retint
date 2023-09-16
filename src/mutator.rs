use crate::gamestate::State;
use std::{
    error::Error,
    io::stdout,
    time::{Duration, Instant},
};

pub fn mutate(state: &mut State, elapsed: Duration) {
    // return state;
    state.scalex = 3;
}
