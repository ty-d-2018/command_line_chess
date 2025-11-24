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
}