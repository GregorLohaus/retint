use crossterm::style::Color;

#[derive(Clone, Copy, Debug)]
pub struct Block {
    //position inside tetromino boundingbox
    pub x: usize,
    pub y: usize,
    pub color: Color,
}

impl Block {
    pub fn set_x(&mut self, x:usize) {
        self.x = x;
    }

    pub fn set_y(&mut self, y:usize) {
        self.y = y;
    }

    pub fn get_x(&self)->usize {
        self.x
    }

    pub fn get_y(&self)->usize {
        self.y
    }
}
#[macro_export]
macro_rules! block {
    ($x:expr,$y:expr,$color:expr) => {
        Block {
            x: $x,
            y: $y,
            color: $color
        }
    };
}
pub struct Tetromino {
    pub tetromino_type: TetrominoType,
    pub blocks: [Block; 5],
    //startposition of boundingbox
    x: usize,
    y: usize,
    xlog: Vec<usize>,
    ylog: Vec<usize>
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
                    block!(0,0,Color::Red),
                    block!(0,1,Color::Red),
                    block!(1,1,Color::Red),
                    block!(2,1,Color::Red),
                    block!(3,1,Color::Red)
                ],
                x: 0,
                y: 0,
                xlog: vec![],
                ylog: vec![]
            },
        }
    }
    pub fn set_x(&mut self, x:usize) {
        self.xlog.push(self.x);
        self.x = x;
    }

    pub fn set_y(&mut self, y:usize) {
        self.ylog.push(self.y);
        self.y = y;
    }

    pub fn get_x(&self)->usize {
        self.x
    }

    pub fn get_y(&self)->usize {
        self.y
    }

    pub fn max_x(&self) -> usize {
        let mut max = 0;
        for block in self.blocks {
            if block.x + self.x > max {
                max = block.x + self.x
            }
        }
        max
    }
    pub fn max_y(&self) -> usize {
        let mut max = 0;
        for block in self.blocks {
            if block.y + self.y > max {
                max = block.y + self.y
            }
        }
        max
    }
}
