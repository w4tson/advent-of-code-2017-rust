use std::collections::HashMap;

#[cfg(test)]
pub mod tests;

fn calc_severity(firewall : HashMap<i32, i32>) -> i32 {
    let width = firewall.keys().max().unwrap_or(&0);
    
    (0..=*width).fold(0, |acc, t| {
        acc +  match is_scan_at_row_zero(&firewall, t, t) {
            true => firewall.get(&t).unwrap() * t,
            false => 0
        }
    })
}

fn is_scan_at_row_zero(firewall: &HashMap<i32, i32>, col: i32, pico_seconds: i32) -> bool {
    match firewall.get(&col) {
        Some(depth) => pico_seconds % ((depth * 2) - 2) == 0,
        None => false
    }
}