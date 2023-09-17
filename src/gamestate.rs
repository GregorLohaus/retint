use std::time::{Duration, Instant};

use crate::tetrominos::{Block, Tetromino, TetrominoType};
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
    //last softdrop activation
    pub lastSDA: Option<Instant>,
    //last moveleft activation
    pub lastMLA: Option<Instant>,
    //last moveright activation
    pub lastMRA: Option<Instant>,
    pub active_tetromino: Option<Tetromino>,
    pub held_tetromino: Option<Tetromino>,
}

impl State {
    pub fn new() -> State {
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
            lastSDA: Option::None,
            lastMLA: Option::None,
            lastMRA: Option::None,
            active_tetromino: Option::None,
            held_tetromino: Option::None,
        }
    }

    pub fn spawn_tetromino(&mut self) {
        match self.active_tetromino.as_ref() {
            Some(_t) => (),
            None => match Tetromino::new(TetrominoType::J) {
                Ok(t) => self.active_tetromino = Some(t),
                Err(_) => (),
            },
        }
    }

    pub fn tetromino_to_board(&mut self) {
        match self.active_tetromino.as_ref() {
            Some(tetromino) => {
                for block in tetromino.blocks {
                    self.board[block.y + tetromino.y][block.x + tetromino.x].color = block.color;
                }
            }
            None => (),
        }
    }
}
