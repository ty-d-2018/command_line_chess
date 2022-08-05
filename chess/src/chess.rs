use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::collections::HashMap;

pub enum PieceType{
    Pawn,
    Rook,
    Bishop,
    Knight,
    Queen,
    King,
}

pub enum MoveType{
    LShape,
    Diagonal,
    UpDown,
    AnyDirectionOne,
    AnyDirectionUnlimited,
    ForwardOne,
}

#[derive(Clone)]
pub struct Point{
    pub x: u32,
    pub y: u32,
}

pub struct ChessPiece{
    piece_type: PieceType,
    move_constraint: MoveType,
    position: Point,
    is_alive: bool,
}

pub struct ChessBoard{
    light_team: HashMap::<String, ChessPiece>,
    dark_team: HashMap::<String, ChessPiece>,
    board_size: Point,
}

impl Point{
    pub fn new(x: u32, y: u32) -> Point{
        Point{
            x,
            y,
        }
    }
}

impl Add for Point{
    type Output = Self;
    fn add(self, other: Self) -> Self{
        Self{
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point{
    type Output = Self;
    fn sub(self, other: Self) -> Self{
        Self{
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul for Point{
    type Output = Self;
    fn mul(self, other: Self) -> Self{
        Self{
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Div for Point{
    type Output = Self;
    fn div(self, rhs: Self) -> Self{
        Self{
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl PieceType{
    pub fn get_move_type(&self) -> MoveType{
        match self{
            PieceType::Pawn => MoveType::ForwardOne,
            PieceType::Rook => MoveType::UpDown,
            PieceType::Bishop => MoveType::Diagonal,
            PieceType::Knight => MoveType::LShape,
            PieceType::Queen => MoveType::AnyDirectionUnlimited,
            PieceType::King => MoveType::AnyDirectionOne,
        }
    }
}


impl ChessPiece{
    pub fn new(piece_type: PieceType) -> ChessPiece{
        let move_type: MoveType = piece_type.get_move_type();
        ChessPiece{
            piece_type,
            move_constraint: move_type,
            position: Point::new(0, 0),
            is_alive: true,
        }
    }
    pub fn set_position(&mut self, new_position: Point){
        self.position = new_position;
    }
    pub fn invert_alive(&mut self){
        self.is_alive = !is_alive;
    }
    pub fn get_move_constraint(&self) -> &MoveType{
        &(self.move_constraint)
    }
    pub fn get_piece_type(&self) -> &PieceType{
        &(self.piece_type)
    }
    pub fn get_position(&self) -> &Point{
        &(self.position)
    }
    pub fn is_it_alive(&self) -> bool{
        self.is_alive
    }
    pub fn move_piece(&mut self, new_point: &Point) -> Result::<&Point, ()>{
        Ok(self.get_position())
    }
    fn check_for_LShape(&self, new_point: &Point) -> bool{
        false
    }
}