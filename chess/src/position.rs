use std::collections::HashMap;

pub enum BoardResponse{
    CannotMoveToPosition,
    ThisCellIsEmpty,
}

pub enum Path{
    UpRight,
    UpLeft,
    DownLeft,
    DownRight,
    RightSideUp,
    RightSideDown,
    UpSideLeft,
    UpSideRight,
    DownSideLeft,
    DownSideRight,
}

pub trait Location{
    fn get_grid_number(&self) -> usize;
    fn get_residents(&self) -> HashMap::<bool, Vec::<impl Location>>;
    fn can_move(&self) -> bool;
    fn flip(&mut self, location: &impl Location) -> Result::<(), BoardResponse>;
    fn get_name(&self) -> String;
}

pub struct Movement{
    relative: usize,
    timetable: bool,
}

impl Movement{
    pub fn new(relative: usize, timetable: bool) -> Movement{
        Movement{
            relative,
            timetable,
        }
    }
    fn find_slant(path: &Path) -> Option::<Movement>{
        match label.as_str(){
            Path::UpRight => Some(Movement::new(0, false)),
            Path::UpLeft => Some(Movement::new(0, false)),
            Path::DownLeft => Some(Movement::new(0, false)),
            Path::DownRight => Some(Movement::new(0, false)),
            _ => None
        }
    }
    fn find_gallop(label &String) -> Movement{
        todo!();
    }
    pub fn slant(piece: &impl Location){
        todo!();
    }
    pub fn lrtd(piece: &impl Location){
        todo!();
    }
    pub fn gallop(piece: &impl Location){
        todo!();
    }
}