extern crate nalgebra as na;
use na::{Matrix2,Matrix1x2, Vector4};

enum KickData {
    JLSTZ,
    I,
    O
}

enum RotationState {
    Spawn,
    Right,
    Left,
    Double
}

enum TetrominoType {
    I,
    O,
    J,
    L,
    S,
    Z
}

enum Rotation {
    Clockwise,
    Counterwise
}


#[derive(Clone)]
#[derive(Copy)]
struct Tile {
    relative_position: Matrix1x2<i8>,
    absolute_position: Matrix1x2<u8>,
    is_origin: bool
}

impl Tile {
    fn new(relative_position:Matrix1x2<i8>,absolute_position:Matrix1x2<u8>,is_origin:bool)-> Self {
        Self {
            absolute_position,
            relative_position,
            is_origin,
        }
    }

    fn set_relative_position(&mut self, p:Matrix1x2<i8>)-> &mut Self {
        self.relative_position = p;
        return self;
    }

    fn set_origin_position(&mut self,pos:Matrix1x2<u8>,t:&mut Tetromino)-> &mut Self {
        if self.is_origin {
            self.absolute_position = self.absolute_position + pos;
            //TODO not sure about this yet 1. requires to pass mutable ref to t 2. is technically a side effect but might be desirable
            t.global_origintile_position = t.global_origintile_position + pos;
        }
        return self;
    }

    fn set_absolute_position(&mut self,t:&Tetromino)-> &mut Self {
        if !self.is_origin {
            self.absolute_position = self.absolute_position + t.global_origintile_position;
        }
        return self;
    }

    fn get_relative_position(&self)->Matrix1x2<i8> {
       return self.relative_position;
    }

    fn get_absolute_position(&self)->Matrix1x2<u8> {
       return self.absolute_position;
    }

    fn rotate(&mut self, rotation:Rotation,t:&Tetromino)-> &Self {
        match rotation {
            Rotation::Clockwise => self.set_relative_position(self.get_relative_position() * Matrix2::new(0,1,-1,0)).set_absolute_position(&t),
            Rotation::Counterwise => self.set_relative_position(self.relative_position * Matrix2::new(0, -1, 1, 0)).set_absolute_position(&t)
        }
    }
}

struct Tetromino {
    kind: TetrominoType,
    global_origintile_position: Matrix1x2<u8>,
    tiles: Matrix2<Tile>,
    rotation_state: RotationState,
    kick_data: KickData
}

trait TetrominoBuilder {
    //create new Tetromino in its spawn state
    fn new()-> Tetromino;
}

// should probably refactor this somehow, but how with no inherritance ??? 
struct I;
impl TetrominoBuilder for I {
    fn new() -> Tetromino {
        let tetromino = Tetromino {
            kind: TetrominoType::I,
            global_origintile_position: Matrix1x2::new(5,0),
            tiles: Matrix2::new(
                Tile::new(Matrix1x2::new(-1,0),Matrix1x2::new(4,0),false),
                Tile::new(Matrix1x2::new(0,0),Matrix1x2::new(5,0),true),
                Tile::new(Matrix1x2::new(1,0),Matrix1x2::new(6,0),false),
                Tile::new(Matrix1x2::new(2,0),Matrix1x2::new(7,0),false),
            ),
            kick_data: KickData::I,
            rotation_state: RotationState::Spawn
        };
        return tetromino;
    }
}

struct O;
impl TetrominoBuilder for O {
    fn new() -> Tetromino {
        let tetromino = Tetromino {
            kind: TetrominoType::O,
            global_origintile_position: Matrix1x2::new(4,0),
            tiles: Matrix2::new( 
                Tile::new(Matrix1x2::new(0,0),Matrix1x2::new(4,0),true),
                Tile::new(Matrix1x2::new(1,0),Matrix1x2::new(5,0),false),
                Tile::new(Matrix1x2::new(0,1),Matrix1x2::new(4,1),false),
                Tile::new(Matrix1x2::new(-1,0),Matrix1x2::new(5,1),false),
            ),
            kick_data: KickData::O,
            rotation_state: RotationState::Spawn
        };
        return tetromino;
    }
}

struct J;
impl TetrominoBuilder for J {
    fn new()-> Tetromino {
        let tetromino = Tetromino {
            kind: TetrominoType::J,
            global_origintile_position: Matrix1x2::new(5,1),
            tiles: Matrix2::new( 
                Tile::new(Matrix1x2::new(-1,-1),Matrix1x2::new(4,0),false),
                Tile::new(Matrix1x2::new(-1,0),Matrix1x2::new(4,1),false),
                Tile::new(Matrix1x2::new(0,0),Matrix1x2::new(5,1),true),
                Tile::new(Matrix1x2::new(1,0),Matrix1x2::new(6,1),false),
            ),
            kick_data: KickData::JLSTZ,
            rotation_state: RotationState::Spawn
        };
        return tetromino;
    }
}

struct L;
impl TetrominoBuilder for L {
    fn new()-> Tetromino {
        let tetromino = Tetromino {
            kind: TetrominoType::L,
            global_origintile_position: Matrix1x2::new(4,1),
            tiles: Matrix2::new( 
                Tile::new(Matrix1x2::new(-1,0),Matrix1x2::new(4,0),false),
                Tile::new(Matrix1x2::new(0,0),Matrix1x2::new(4,1),true),
                Tile::new(Matrix1x2::new(1,0),Matrix1x2::new(5,1),false),
                Tile::new(Matrix1x2::new(2,0),Matrix1x2::new(6,1),false),
            ),
            kick_data: KickData::JLSTZ,
            rotation_state: RotationState::Spawn
        };
        return tetromino;
    }
}

struct S;
impl TetrominoBuilder for S {
    fn new()-> Tetromino {
        let tetromino = Tetromino {
            kind: TetrominoType::S,
            global_origintile_position: Matrix1x2::new(5,1),
            tiles: Matrix2::new( 
                Tile::new(Matrix1x2::new(-1,0),Matrix1x2::new(4,1),false),
                Tile::new(Matrix1x2::new(0,0),Matrix1x2::new(5,1),true),
                Tile::new(Matrix1x2::new(1,0),Matrix1x2::new(5,0),false),
                Tile::new(Matrix1x2::new(2,0),Matrix1x2::new(6,0),false),
            ),
            kick_data: KickData::JLSTZ,
            rotation_state: RotationState::Spawn
        };
        return tetromino;
    }
}

struct Z;
impl TetrominoBuilder for Z {
    fn new()-> Tetromino {
        let mut origin_position = Matrix1x2::new(5,1);
        let tetromino = Tetromino {
            kind: TetrominoType::S,
            global_origintile_position: origin_position,
            tiles: Matrix2::new( 
                Tile::new(Matrix1x2::new(-1,-1),Matrix1x2::new(origin_position.x -1,origin_position.y -1),false),
                Tile::new(Matrix1x2::new(0,-1),pos_offsets[1]+origin_position,false),
                Tile::new(Matrix1x2::new(1,0),origin_position,true),
                Tile::new(Matrix1x2::new(0,0),origin_position,false),
            ),
            kick_data: KickData::JLSTZ,
            rotation_state: RotationState::Spawn
        };
        return tetromino;
    }
}

