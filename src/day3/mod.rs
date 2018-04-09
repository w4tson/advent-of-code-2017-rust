pub mod part1;
pub mod part2;
#[cfg(test)]
pub mod tests;

#[derive(Debug, Clone)]
pub struct Location {
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

impl Location {
    
    pub fn locations_around_me(&self) -> Vec<Location> {
        vec!(
            Location { x: self.x + 1, y: self.y     },
            Location { x: self.x + 1, y: self.y + 1 },
            Location { x: self.x,     y: self.y + 1 },
            Location { x: self.x - 1, y: self.y + 1 },
            Location { x: self.x - 1, y: self.y     },
            Location { x: self.x - 1, y: self.y - 1 },
            Location { x: self.x,     y: self.y - 1 },
            Location { x: self.x + 1, y: self.y - 1 }
        )
    }
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
            Direction::EAST => Location { x: this.location.x + 1, y: this.location.y },
            Direction::WEST => Location { x: this.location.x - 1, y: this.location.y },
        };

        let new_locstat = LocStat {
            location: new_location,
            direction: self.direction.clone()
        };
//        println!("advancing to {:?}", l);

        new_locstat
    }

    fn next(&self, index: u32) -> LocStat {
        let this = self;
        match should_turn(index) {
            true => this.turn_and_advance(),
            _ => this.advance()
        }
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

fn calc_corner(num : u32) -> u32 {
    let ceil_square = (num as f64).sqrt().ceil() as u32;

    let sqrt_of_bottom_corner = match ceil_square {
        x if x % 2 == 0 => x+1,
        _ => ceil_square
    };

    sqrt_of_bottom_corner.pow(2)
}

fn ring(num: u32) -> u32 {
    (((calc_corner(num) as f64).sqrt() as u32) + 1) / 2
}

fn width(num: u32) -> u32 {
    ring(num)* 2 -1
}

fn calc_all_corners(num : u32) -> Vec<u32> {
    let br = calc_corner(num);
    let side = width(num) -1;
    vec!(br, br-(2*side), br-(3*side), br-side)
}

fn corner_for_ring(ring: u32) -> u32 {
    let t = ring as i32 * 2 -1;
    (t * t) as u32
}

fn should_turn(num: u32) -> bool {
    let three_corners = &calc_all_corners(num)[1..4];
    let previous_ring = ring(num) - 1;

    match &num {
        1 | 2 => true,
        x if three_corners.contains(&x) => true,
        y if *y == corner_for_ring(previous_ring) + 1 => true,
        _ => false
    }
}
