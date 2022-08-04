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