use std::collections::HashMap;

pub enum BoardResponse{
    CannotMoveToPosition,
    ThisCellIsEmpty,
}

pub trait Location{
    fn get_grid_number(&self) -> usize;
    fn get_residents(&self) -> HashMap::<bool, Vec::<impl Location>>;
    fn can_move(&self) -> bool;
    fn flip(&mut self, location: &impl Location) -> Result::<(), BoardResponse>;
    fn get_name(&self) -> String;
}

pub struct Movement;

impl Movement{
    fn find_slant(label: &String) -> Option::<(bool, bool)>{
        match label.as_str(){
            "up-right" => Some((true, true)),
            "up-left" => Some((false, true)),
            "down-left" => Some((false, false)),
            "down-right" => Some((true, false)),
            _ => None
        }
    }
    pub fn slant(piece: &impl Location){}
}