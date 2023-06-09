
#[derive(Clone)]
#[derive(Copy)]
enum SquareState {
    Filled(bool),
}

#[derive(Clone)]
#[derive(Copy)]
pub struct Board {
    board: [[SquareState;10];20],
    scalex: u8,
    scaley: u8,
    size: u8,
}

impl Board {
    fn new(&mut self)->Board {
        let mut board = Board { 
            board : [[SquareState::Filled(false);10];20], 
            scalex : 2,
            scaley : 1,
            size : 30
        };
        return board;
    }

    fn set_square_state(&mut self,x:usize,y:usize,state:SquareState) {
        self.board[y][x] = state;
    }
}