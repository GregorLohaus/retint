use crossterm::style::Color;

#[derive(Clone, Copy, Debug)]
pub struct Block {
    //position inside tetromino boundingbox
    pub x: usize,
    pub y: usize,
    pub color: Color,
}

pub struct Tetromino {
    pub tetromino_type: TetrominoType,
    pub blocks: [Block; 5],
    //bounding box width and height
    pub width: usize,
    pub height: usize,
    //startposition of boundingbox
    pub x: usize,
    pub y: usize,
}

pub enum TetrominoType {
    I,
    T,
    J,
    L,
    S,
    Z,
}

impl Tetromino {
    pub fn new(t: TetrominoType) -> Result<Tetromino, &'static str> {
        match t {
            TetrominoType::J => Ok(Tetromino {
                tetromino_type: TetrominoType::J,
                blocks: [
                    Block {
                        x: 0,
                        y: 0,
                        color: Color::Red,
                    },
                    Block {
                        x: 0,
                        y: 1,
                        color: Color::Red,
                    },
                    Block {
                        x: 1,
                        y: 1,
                        color: Color::Red,
                    },
                    Block {
                        x: 2,
                        y: 1,
                        color: Color::Red,
                    },
                    Block {
                        x: 3,
                        y: 1,
                        color: Color::Red,
                    },
                ],
                width: 4,
                height: 4,
                x: 0,
                y: 0,
            }),
            _ => Err("Todo"),
        }
    }
}
