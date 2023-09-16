use std::time::Duration;

use crate::tetrominos::Block;
use crossterm::style::Color;

pub enum Action {
    Hold,
    RotateR,
    RotateL,
    SoftDropA,
    SoftDropD,
    HardDrop,
    Flip,
    MoveLeftA,
    MoveLeftD,
    MoveRightA,
    MoverRightD,
}

pub struct State {
    pub board: [[Block; 10]; 20],
    pub scalex: usize,
    pub eventqueue: Vec<Action>,
    // Automatic Repeat Rate:
    // the speed at which tetrominoes move when holding down movement keys.
    pub auto_repeat_rate: Duration,

    // Delayed Auto Shift:
    // the time between the initial keypress and the start of its automatic repeat movement.
    pub delayed_auto_shit: Duration,
    pub soft_drop_step_time: Duration,
}

pub fn create_state() -> State {
    let mut board = [[Block {
        x: 0,
        y: 0,
        color: Color::Black,
    }; 10]; 20];
    for i in 0..=19 {
        for n in 0..=9 {
            board[i][n] = Block {
                x: n,
                y: i,
                color: Color::White,
            };
        }
    }
    State {
        board,
        scalex: 2,
        eventqueue: vec![],
        auto_repeat_rate: Duration::from_millis(0),
        delayed_auto_shit: Duration::from_millis(96),
        soft_drop_step_time: Duration::from_millis(0),
    }
}
