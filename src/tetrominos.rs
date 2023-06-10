extern crate nalgebra as na;
use na::{Matrix2,Matrix1x2};

enum KickData {
    JLSTZ,
    I
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

    fn set_absolutee_position(&mut self,t:&Tetromino)-> &mut Self {
        self.absolute_position = self.absolute_position + t.global_origintile_position;
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
            Rotation::Clockwise => self.set_relative_position(self.get_relative_position() * Matrix2::new(0,1,-1,0)).set_absolutee_position(&t),
            Rotation::Counterwise => self.set_relative_position(self.relative_position * Matrix2::new(0, -1, 1, 0)).set_absolutee_position(&t)
        }
    }
}


struct Tetromino<'a> {
    kind: TetrominoType,
    global_origintile_position: Matrix1x2<u8>,
    tiles: [&'a Tile;5],
    rotation_state: RotationState,
    kick_data: KickData
}

trait TetorMinoBuilder {
    //create new Tetromino in its spawn state
    fn new<'a>()-> Box<Tetromino<'a>>;
}
struct I;
impl TetorMinoBuilder for I {
    fn new<'a>() -> Box<Tetromino<'a>> {
        let tetromino = Tetromino {
            kind: TetrominoType::I,
            global_origintile_position: Matrix1x2::new(5,0),
            tiles: [ 
                &Tile::new(Matrix1x2::new(-1,0),Matrix1x2::new(4,0),false),
                &Tile::new(Matrix1x2::new(-1,0),Matrix1x2::new(4,0),true),
                &Tile::new(Matrix1x2::new(-1,0),Matrix1x2::new(4,0),false),
                &Tile::new(Matrix1x2::new(-1,0),Matrix1x2::new(4,0),false),
                &Tile::new(Matrix1x2::new(-1,0),Matrix1x2::new(4,0),false),
            ],
            kick_data: KickData::I,
            rotation_state: RotationState::Spawn
        };
        return Box::new(tetromino);
    }
}