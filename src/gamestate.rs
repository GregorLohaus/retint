use crate::tetrominos::Block;
use crossterm::style::Color;

pub enum Action {
    Hold,
    RotateR,
    RotateL,
    SoftDrop,
    HardDrop,
    Flip,
}

pub struct State {
    pub board: [[Block; 10]; 20],
    pub scalex: usize,
    pub eventqueue: Vec<Action>,
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
                x: u32::try_from(n).unwrap(),
                y: u32::try_from(i).unwrap(),
                color: Color::White,
            };
        }
    }
    State {
        board,
        scalex: 2,
        eventqueue: vec![],
    }
}
