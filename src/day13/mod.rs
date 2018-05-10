use std::collections::HashMap;

#[cfg(test)]
pub mod tests;

fn calc_severity(firewall : &HashMap<i32, i32>) -> i32 {
    let width = firewall.keys().max().unwrap_or(&0);
    
    (0..=*width).fold(0, |acc, t| {
        acc +  match is_scan_at_row_zero(&firewall, t, t) {
            true => firewall.get(&t).unwrap() * t,
            false => 0
        }
    })
}

fn calc_delay_to_avoid_getting_caught(firewall : &HashMap<i32, i32>) -> i32 {
    let width = firewall.keys().max().unwrap_or(&0);

    (0..).skip_while(|i| is_caught(&firewall, *width, *i)).next().unwrap_or(-1)
}

fn is_caught(firewall : &HashMap<i32, i32>, width: i32, delay: i32) -> bool {
    (delay..=delay+width)
        .any(|t| is_scan_at_row_zero(&firewall, t - delay, t))
}

fn is_scan_at_row_zero(firewall: &HashMap<i32, i32>, col: i32, pico_seconds: i32) -> bool {
    match firewall.get(&col) {
        Some(depth) => pico_seconds % ((depth * 2) - 2) == 0,
        None => false
    }
}