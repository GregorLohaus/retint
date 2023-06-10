extern crate nalgebra as na;
use na::{Matrix2,Matrix2x1};
#[derive(Clone)]
#[derive(Copy)]
enum Color {
    Cyan,
    Yellow,
    Purple,
    Green,
    Blue,
    Red,
    Orange,
    None
}

#[derive(Clone)]
#[derive(Copy)]
enum SquareState {
    Filled(bool,Color),
}


pub struct Board {
    grid: [[SquareState;10];20],
    scalex: u8,
    scaley: u8,
    size: u8,
}

impl Board {
    fn new(&mut self)->Board {
        let mut board = Board { 
            grid : [[SquareState::Filled(false,Color::None);10];20], 
            scalex : 2,
            scaley : 1,
            size : 30
        };
        return board;
    }

    fn set_square_state(&mut self,x:usize,y:usize,state:SquareState) {
        self.grid[y][x] = state;
    }
}

pub struct State {
    board: Board,
}