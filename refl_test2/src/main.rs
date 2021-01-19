// https://users.rust-lang.org/t/var-dump-and-print-r-alternative-in-rust/36944/2

#![allow(unused_imports)]

extern crate serde_derive;

extern crate serde;
extern crate serde_value;

use std::fmt::Debug;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_value::Value;

#[derive(Debug)]
//#[derive(Serialize)]
struct Point {
	x: i32,
	y: i32
}

#[derive(Debug)]
struct Point2 {
	x: i32,
	y: i32
}

impl From<Point> for Point2 {
	fn from(inval: Point) -> Self {
		Point2 { x: inval.x, y: inval.y }
	}
}


fn main() {
	println!("Hello, world!");
	let p = Point {x:12, y:23};
	println!("{:?}",  p);
	println!("{:?}",  Point2::from(p));
}
