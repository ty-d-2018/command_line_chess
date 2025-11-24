use std::collections::HashMap;

pub trait Location{
    fn get_grid_number(&self) -> usize;
    fn get_residents(&self) -> HashMap::<bool, Vec::<impl Location>>;
    fn can_move(&self) -> Option::<bool>;
}