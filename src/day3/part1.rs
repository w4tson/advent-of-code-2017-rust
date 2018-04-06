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
        
        let l = LocStat { 
            location: new_location,
            direction: self.direction.clone()
        };
//        println!("advancing to {:?}", l);
        
        l
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

pub fn calc_corner(num : u32) -> u32 {
    let ceil_square = (num as f64).sqrt().ceil() as u32;

    let sqrt_of_bottom_corner = match ceil_square {
        x if x % 2 == 0 => x+1,
        _ => ceil_square
    };

    sqrt_of_bottom_corner.pow(2)
}

pub fn ring(num: u32) -> u32 {
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

fn should_turn(num: u32) -> bool {
    let three_corners = &calc_all_corners(num)[1..4];
    let previous_ring = ring(num) -1;
    let t = previous_ring as i32 * 2 -1;
    let next_ring = (t * t + 1 ) as u32; 
    match &num {
        1 | 2 => true,
        x if three_corners.contains(&x) => true,
        y if *y == next_ring => true,
        _ => false
    }
}

pub fn solve(num: u32) -> u32 {
    let mut locstat = LocStat { 
        location: Location{ x : 0, y: 0},
        direction: Direction::SOUTH
    };

    for i in 1..num {
        locstat = match should_turn(i) {
            true => locstat.turn_and_advance(),
            _    => locstat.advance()
        };
    }
    
    println!("distance : {}   {:?}", locstat.distance(), locstat);
    locstat.distance()
//    37 36  35  34  33  32 31
//    38 17  16  15  14  13 30
//    39 18   5   4   3  12 29
//    40 19   6   1   2  11 28
//    41 20   7   8   9  10 27
//    42 21  22  23  24  25 26
//    43 44  45  46  47  48 49

//    4 3 2 3 4
//    3 2 1 2 3
//    2 1 0 1 2
//    3 2 1 2 3
//    4 3 2 3 4
    
}