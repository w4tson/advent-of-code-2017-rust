pub mod part1;
#[cfg(test)]
pub mod tests;

#[derive(Debug, Clone)]
struct Location {
    x : i32,
    y : i32
}

#[derive(Debug)]
struct LocStat {
    location : Location,
    direction : Direction
}

#[derive(Debug, Clone)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST
}


impl LocStat {

    fn turn_and_advance(&self) -> LocStat {
        let new_direction = self.new_direction();
        LocStat {
            location: self.location.clone(),
            direction: new_direction
        }.advance()
    }

    fn advance(&self) -> LocStat {
        let this = self;

        let new_location = match self.direction {
            Direction::NORTH => Location { x: this.location.x, y: this.location.y + 1 },
            Direction::SOUTH => Location { x: this.location.x, y: this.location.y - 1 },
            Direction::EAST  => Location { x: this.location.x + 1, y: this.location.y },
            Direction::WEST  => Location { x: this.location.x - 1, y: this.location.y },
        };

        let new_locstat = LocStat {
            location: new_location,
            direction: self.direction.clone()
        };
//        println!("advancing to {:?}", l);

        new_locstat
    }

    fn new_direction(&self) -> Direction {
        match self.direction {
            Direction::NORTH => Direction::WEST,
            Direction::WEST => Direction::SOUTH,
            Direction::SOUTH => Direction::EAST,
            Direction::EAST => Direction::NORTH
        }
    }

    fn distance(&self) -> u32 {
        (self.location.x.abs() + self.location.y.abs()) as u32
    }
}

