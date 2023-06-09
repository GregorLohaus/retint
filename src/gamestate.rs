
#[derive(Clone)]
#[derive(Copy)]
enum SquareState {
    Filled(bool),
}

pub struct Board {
    board: [[SquareState;10];20],
    scalex: u8,
    scaley: u8,
}

impl Board {
    fn new(&mut self) {
        let mut board = Board { 
            board : [[SquareState::Filled(false);10];20], 
            scalex : 2,
            scaley : 1
        };
    }

    fn set_square_state(&mut self,x:usize,y:usize,state:SquareState) {
        self.board[y][x] = state;
    }
}