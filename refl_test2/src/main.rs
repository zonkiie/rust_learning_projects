// https://users.rust-lang.org/t/var-dump-and-print-r-alternative-in-rust/36944/2
// https://stackoverflow.com/questions/38111486/how-do-i-iterate-over-elements-of-a-struct-in-rust
// https://stackoverflow.com/questions/62363984/how-to-flatten-a-vec-field-when-serializing-a-struct-with-serde
// https://github.com/servo/bincode
// https://github.com/tafia/quick-xml
// https://riptutorial.com/rust/topic/1170/serde

#![allow(unused_imports)]

extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_value;
extern crate quick_xml;

use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use serde_value::Value;
use quick_xml::de::{from_str, DeError};
use quick_xml::se::to_string;

//#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[derive(Serialize, Deserialize, PartialEq)]
struct Point {
	x: i32,
	y: i32
}

#[derive(Serialize, Deserialize, PartialEq)]
struct Point2 {
	x: i32,
	y: i32
}

#[derive(Serialize, Deserialize, PartialEq)]
struct Line {
	p1: Point,
	p2: Point
}

#[derive(Serialize, Deserialize, PartialEq)]
struct World(Vec<Point>);

impl From<Point> for Point2 {
	fn from(inval: Point) -> Self {
		Point2 { x: inval.x, y: inval.y }
	}
}

impl From<&Point> for Point2 {
	fn from(inval: &Point) -> Self {
		Point2 { x: inval.x, y: inval.y }
	}
}

fn main() {
	//let p = Point {x:12, y:23};
	//let str = to_string(&p).unwrap();
	//println!("String: {:?}", str);
	let str = to_string(&Point {x:12, y:23}).unwrap();
	println!("String: {}", str);
	let str = to_string(&Line {p1: Point {x:1, y:1}, p2: Point {x:15, y:15}}).unwrap();
	println!("Line: {}", str);
}
