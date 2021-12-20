#[allow(unused_imports)]
use std::ops::Range;
#[allow(unused_imports)]
use std::cmp::Ordering;

fn find_next_slot<i64>(ranges: &Vec<Range<i64>>, cand: &i64) -> i64 {
	//let mut sorted_ranges = ranges.sort_by(|a, b| a.end.cmp(b.start).unwrap());
	let mut sorted_ranges = ranges.sort_by(|a, b| a.cmp(b).unwrap());
	let found:i64 = ranges[0].start;
	found
}

/*fn find_next_slot<T>(ranges: [&Range<T>], cand: T) ->T {
	let mut sorted_ranges = ranges.sort_by(|a, b| a.cmp(b).unwrap());
}*/

/*fn find_previous_slot<T>(ranges: [&Range<T>], cand: T) ->T {
	
}*/
