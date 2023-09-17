use crossterm::style::Color;

#[derive(Clone, Copy, Debug)]
pub struct Block {
    //position inside tetromino boundingbox
    pub x: usize,
    pub y: usize,
    pub color: Color,
}

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
pub enum TetrominoType {
    I,
    T,
    J,
    L,
    S,
    Z,
}

impl Tetromino {
    pub fn new(t: TetrominoType) -> Tetromino {
        match t {
            //TetrominoType::J
            _ => Tetromino {
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
            },
        }
    }

    pub fn max_x(&self) -> usize {
        let mut max = 0;
        for block in self.blocks {
            if block.x > max {
                max = block.x
            }
        }
        max
    }
    pub fn max_y(&self) -> usize {
        let mut max = 0;
        for block in self.blocks {
            if block.y > max {
                max = block.y
            }
        }
        max
    }
}
