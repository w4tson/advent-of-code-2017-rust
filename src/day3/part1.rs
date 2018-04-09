use day3::LocStat;
use day3::Location;
use day3::Direction;
use day3::should_turn;

pub fn solve(num: u32) -> u32 {
    let mut locstat = LocStat { 
        location: Location { x : 0, y: 0 },
        direction: Direction::SOUTH
    };

    for i in 1..num {
        locstat = locstat.next(i);
    }
    
    println!("distance : {}   {:?}", locstat.distance(), locstat);
    locstat.distance()
}

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