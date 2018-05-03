
#[cfg(test)]
pub mod tests;

#[derive(Debug, Clone)]
struct HexCoord {
    x: i32,
    y: i32, 
    z: i32
}


impl HexCoord {
    
    fn next_hex(&self, direction: &str) -> HexCoord {
        match &direction {
            &"n"  => HexCoord { x: self.x,     y: self.y + 1, z: self.z - 1 },
            &"ne" => HexCoord { x: self.x + 1, y: self.y,     z: self.z - 1 },
            &"se" => HexCoord { x: self.x + 1, y: self.y - 1, z: self.z },
            &"s"  => HexCoord { x: self.x,     y: self.y - 1, z: self.z + 1 },
            &"sw" => HexCoord { x: self.x - 1, y: self.y,     z: self.z + 1 },
            &"nw" => HexCoord { x: self.x - 1, y: self.y + 1, z: self.z     },
            _ => panic!("Incorrect direction")
        }
    }

    fn distance_from(&self, from: &HexCoord) -> i32 {
        let x = from.x - self.x;
        let y = from.y - self.y;
        let z = from.z - self.z;
        return (x.abs() + y.abs() + z.abs()) / 2
    }
}

