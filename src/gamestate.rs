use core::fmt;
use std::time::{Duration, Instant};

use crate::tetrominos::{Block, Tetromino, TetrominoType};
use crossterm::style::Color;
#[derive(Debug)]
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

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::Hold => write!(f, "Hold"),
            Action::RotateR => write!(f, "RotateR"),
            Action::RotateL => write!(f, "RotateL"),
            Action::SoftDropA => write!(f, "SoftDropA"),
            Action::SoftDropD => write!(f, "SoftDropD"),
            Action::HardDrop => write!(f, "HardDrop"),
            Action::Flip => write!(f, "Flip"),
            Action::MoveLeftA => write!(f, "MoveLeftA"),
            Action::MoveLeftD => write!(f, "MoveLeftD"),
            Action::MoveRightA => write!(f, "MoveRightA"),
            Action::MoverRightD => write!(f, "MoverRightD"),
        }
    }
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

    last_active_tetromino_block_positions: Option<[[usize; 2]; 5]>,
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
            last_active_tetromino_block_positions: None,
        }
    }

    pub fn spawn_tetromino(&mut self) {
        match self.active_tetromino.as_ref() {
            Some(_t) => (),
            None => self.active_tetromino = Some(Tetromino::new(TetrominoType::J)),
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

    fn save_active_tetromino_block_positions(&mut self) {
        match self.last_active_tetromino_block_positions {
            None => self.last_active_tetromino_block_positions = Some([[0; 2]; 5]),
            Some(_) => (),
        }

        match self.active_tetromino.as_ref() {
            Some(t) => {
                for i in 0..t.blocks.len() {
                    self.last_active_tetromino_block_positions.unwrap()[i] =
                        [t.blocks[i].x, t.blocks[i].y]
                }
            }
            None => (),
        }
    }

    fn clear_last_drawn_blocks(&mut self) {
        match self.last_active_tetromino_block_positions {
            None => (),
            Some(ap) => {
                for p in ap {
                    self.board[p[1]][p[0]].color = Color::White
                }
            }
        }
    }

    pub fn move_left(&mut self) {
        if let Some(mut t) = self.active_tetromino {
            if t.x > 0 {
                t.x -= 1;
                self.save_active_tetromino_block_positions();
                self.clear_last_drawn_blocks();
                self.tetromino_to_board();
            }
        }
        // match self.active_tetromino.as_mut() {
        //     Some(t) => {
        //         if t.x > 0 {
        //             (*t).x -= 1;
        //             self.save_active_tetromino_block_positions();
        //             self.clear_last_drawn_blocks();
        //             self.tetromino_to_board();
        //         }
        //     }
        //     None => (),
        // }
    }

    pub fn move_right(&mut self) {
        if let Some(mut t) = self.active_tetromino {
            if t.max_x() < 20 {
                t.x += 1;
                self.save_active_tetromino_block_positions();
                self.clear_last_drawn_blocks();
                self.tetromino_to_board();
            }
        }
    }
}
