#[allow(unused_imports)]
use std::ops::Range;
mod slotfinder;
use slotfinder::*;

fn main() {
    let mut arr:Vec<Range<i64>> = vec![(100..999), (10000..99999), (1000000..9999999), (1..10)];
    println!("Arr: {:?}", arr.to_vec());
    let arr2 = find_next_slot(&mut arr, &100);
    
}
