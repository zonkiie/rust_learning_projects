#[allow(unused_imports)]
use std::ops::Range;
#[allow(unused_imports)]
use std::cmp::Ordering;

pub fn find_next_slot<T: Ord>(ranges: &mut Vec<Range<T>>, cand: &T) -> T {
	//let mut sorted_ranges = ranges.sort_by(|a, b| a.end.cmp(b.start).unwrap());
	let sorted_ranges = ranges.sort();//ranges.sort_by(|a, b| a.cmp(b));
	println!("Sorted Arr: {:?}", sorted_ranges);
	let found:T = ranges[0].start;
	found
}

/*fn find_next_slot<T>(ranges: [&Range<T>], cand: T) ->T {
	let mut sorted_ranges = ranges.sort_by(|a, b| a.cmp(b).unwrap());
}*/

/*fn find_previous_slot<T>(ranges: [&Range<T>], cand: T) ->T {
	
}*/
