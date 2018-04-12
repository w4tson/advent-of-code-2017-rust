use day3::LocStat;
use day3::Location;
use day3::Direction;
use std::collections::HashMap;

#[derive(Clone)]
struct GridCell {
    index: u32,
    value: u32,
    locstat: LocStat
}

impl GridCell {
    fn location(&self) -> Location {
        self.locstat.location.clone()
    }
    
    fn next<'s>(&'s self) -> Box<GridCell> {
        Box::new(GridCell {
            index: self.index +1,
            value : 0,
            locstat: self.locstat.next(self.index)
        })
    }
}

fn calc_new_value(cell: &GridCell, grid: &HashMap<Location, Box<GridCell>>) -> u32 {
    cell.location()
        .locations_around_me()
        .iter()
        .flat_map(|loc| grid.get(loc))
        .map(|g| g.value)
        .sum()
}

pub fn day3part2(target_value: u32) -> u32 {
    
    let origin = Box::new(GridCell {
        index: 1,
        value: 1,
        locstat: LocStat {
            location: Location { x : 0, y: 0 },
            direction: Direction::SOUTH
        }
    });

    let mut grid : HashMap<Location, Box<GridCell>> = HashMap::new();

    let mut curr = origin.clone();
    
    grid.insert(origin.location(), origin);

    while curr.value < target_value {
        let mut cell = curr.next();
        cell.value = calc_new_value(&cell, &grid);
        curr = cell.clone();
        
        grid.insert(cell.location(),cell);
    }
    
    curr.value
}
