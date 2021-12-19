#[allow(unused_imports)]
use std::ops::Range;

fn find_next_slot<T>(ranges: &Vec<Range<T>>, cand: &T) -> T {
	let mut sorted_ranges = ranges.sort();
	let found:T = ranges[0].start;
	found
}

/*fn find_previous_slot<T>(ranges: [&Range<T>], cand: T) ->T {
	
}*/
