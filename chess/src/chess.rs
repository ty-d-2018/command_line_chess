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

#[derive(Clone, PartialEq, Debug)]
pub struct Point{
    pub x: i32,
    pub y: i32,
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
    pub fn new(x: i32, y: i32) -> Point{
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
        self.is_alive = !(self.is_alive);
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
    pub fn move_piece(&mut self, new_point: &Point) -> Option::<&Point>{
        let can_piece_move_to_point: bool = match self.piece_type{
            PieceType::Knight => {self.can_knight_move(new_point)},
            _ => false,
        };
        if !can_piece_move_to_point{
            return None;
        }
        self.set_position(new_point.clone());
        Some(self.get_position())
    }
    fn can_knight_move(&self, new_point: &Point) -> bool{
        match self.piece_type{
            PieceType::Knight => (),
            _ => {return false;},
        };
        let x2: i32 = new_point.x;
        let y2: i32 = new_point.y;
        let x1: i32 = self.position.x;
        let y1: i32 = self.position.y;
        let is_it_left = match ChessPiece::where_is_axis_2(x1, x2){
            Some(v) => !v,
            None => {return false;},
        };
        let is_it_above = match ChessPiece::where_is_axis_2(y1, y2){
            Some(v) => v,
            None => {return false;},
        };
        let does_the_point_match: bool = ChessPiece::does_l_shapes_match_point_2(is_it_left, is_it_above, x1, y1, new_point);

        does_the_point_match
    }
    fn where_is_axis_2(axis_1: i32, axis_2: i32) -> Option::<bool>{
        if axis_2 - axis_1 > 0{
            return Some(true);
        }else if axis_2 - axis_1 < 0{
            return Some(false);
        }else{
            return None;
        }
    }

    fn does_l_shapes_match_point_2(is_it_left: bool, is_it_above: bool, x1: i32, y1: i32, point_2: &Point) -> bool{
        let mut l_shape_vertical: Point = Point::new(0, 0);
        let mut l_shape_horizontal: Point = Point::new(0, 0);
        if is_it_above{
            if is_it_left{
                l_shape_vertical.x = -1;
                l_shape_vertical.y = 2;
                l_shape_horizontal.x = -2;
                l_shape_horizontal.y = 1;
            }else{
                l_shape_vertical.x = 1;
                l_shape_vertical.y = 2;
                l_shape_horizontal.x = 2;
                l_shape_horizontal.y = 1;
            }
        }else{
            if is_it_left{
                l_shape_vertical.x = -1;
                l_shape_vertical.y = -2;
                l_shape_horizontal.x = -2;
                l_shape_horizontal.y = -1;
            }else{
                l_shape_vertical.x = 1;
                l_shape_vertical.y = -2;
                l_shape_horizontal.x = 2;
                l_shape_horizontal.y = -1;
            }
        }
        l_shape_vertical.x = x1 + l_shape_vertical.x;
        l_shape_horizontal.x = x1 + l_shape_horizontal.x;
        l_shape_vertical.y = y1 + l_shape_vertical.y;
        l_shape_horizontal.y = y1 + l_shape_horizontal.y;
        if l_shape_vertical == *(point_2) || l_shape_horizontal == *(point_2){
            return true;
        }else{
            return false;
        } 
    }
}