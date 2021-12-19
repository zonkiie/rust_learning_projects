#[allow(unused_imports)]
use std::ops::Range;
mod slotfinder;

fn main() {
    let arr = vec![(100..999), (10000..99999), (1000000..9999999)];
    println!("Arr: {:?}", arr.to_vec());
    
}
