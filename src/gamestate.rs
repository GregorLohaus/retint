use crate::tetrominos::Block;
use crossterm::style::Color;
pub struct State {
    pub board: [[Block; 10]; 20],
    pub scalex: usize,
}

pub fn create_state() -> State {
    let mut board = [[Block {
        x: 0,
        y: 0,
        color: Color::Black,
    }; 10]; 20];
    for i in 1..20 {
        for n in 1..10 {
            board[i][n] = Block {
                x: u32::try_from(n).unwrap(),
                y: u32::try_from(i).unwrap(),
                color: Color::Black,
            }
        }
    }
    State { board, scalex: 2 }
}
