use core::{fmt, panic};
use std::time::{Duration, Instant};

use crate::block;
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
    pub scale_x: usize,
    pub event_queue: Vec<Action>,
    // Automatic Repeat Rate:
    // the speed at which tetrominos move when holding down movement keys.
    pub auto_repeat_rate: Duration,
    // Delayed Auto Shift:
    // the time between the initial keypress and the start of its automatic repeat movement.
    pub delayed_auto_shit: Duration,
    pub soft_drop_step_time: Duration,
    pub last_soft_drop_activation: Option<Instant>,
    pub soft_drop_active: bool,
    pub last_move_left_activation: Option<Instant>,
    pub move_left_active: bool,
    pub last_move_right_activation: Option<Instant>,
    pub move_right_active: bool,
    pub active_tetromino: Option<Tetromino>,
    pub held_tetromino: Option<Tetromino>,
    pub tetromino_x_log: Vec<usize>,
    pub last_active_tetromino_block_positions: Option<[[usize; 2]; 5]>,
    pub fps: u64,
}

impl State {
    pub fn new() -> State {
        let mut board = [[block!(0, 0, Color::Black); 10]; 20];
        for i in 0..=19 {
            for n in 0..=9 {
                board[i][n] = block!(n, i, Color::White);
            }
        }
        State {
            board,
            scale_x: 2,
            event_queue: vec![],
            auto_repeat_rate: Duration::from_millis(0),
            delayed_auto_shit: Duration::from_millis(96),
            soft_drop_step_time: Duration::from_millis(0),
            last_soft_drop_activation: Option::None,
            soft_drop_active: false,
            last_move_left_activation: Option::None,
            move_left_active: false,
            last_move_right_activation: Option::None,
            move_right_active: false,
            active_tetromino: Option::None,
            held_tetromino: Option::None,
            last_active_tetromino_block_positions: None,
            tetromino_x_log: vec![],
            fps: 0,
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
                    let y = block.get_y() + tetromino.get_y();
                    let x = block.get_x() + tetromino.get_x();
                    self.board[y][x].color = block.color;
                }
            }
            None => (),
        }
    }

    fn save_active_tetromino_block_positions(&mut self) {
        match self.active_tetromino.as_ref() {
            Some(t) => {
                let mut last_active_tetromino_block_positions = [[0; 2]; 5];
                for i in 0..t.blocks.len() {
                    last_active_tetromino_block_positions[i] = [
                        t.blocks[i].get_x() + t.get_x(),
                        t.blocks[i].get_y() + t.get_y(),
                    ];
                }
                self.last_active_tetromino_block_positions =
                    Some(last_active_tetromino_block_positions);
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
    //TODO cast left ray
    pub fn move_left(&mut self) {
        if let Some(ref mut t) = &mut self.active_tetromino {
            self.save_active_tetromino_block_positions();
        }
        if let Some(ref mut t) = &mut self.active_tetromino {
            if t.get_x() > 0 {
                let x = t.get_x();
                t.set_x(x - 1);
                self.clear_last_drawn_blocks();
                self.tetromino_to_board();
            }
        }
    }
    //TODO cast right ray
    pub fn move_right(&mut self) {
        if let Some(ref mut t) = &mut self.active_tetromino {
            self.save_active_tetromino_block_positions();
        }
        if let Some(ref mut t) = &mut self.active_tetromino {
            let mx = t.max_x();
            if mx < 9 {
                let x = t.get_x();
                t.set_x(x + 1);
                self.clear_last_drawn_blocks();
                self.tetromino_to_board();
            }
        }
    }
}
