extern crate gendata;

use std::fmt;
use gendata::geom::point::*;
use gendata::geom::rect::*;

fn main() {
	let r1 = Rect::from_points(
		Point::new( 1., 3. ),
		Point::new( 2., 0. )
	);

	let r2 = Rect::from_points(
		Point::new( 0., 2. ),
		Point::new( 3., 1. )
	);

	let r3 = r1.get_intersect(&r2).unwrap();
	
	printRect(&r1);
	printRect(&r2);
	printRect(&r3);
// 	println!("{}", r1);
// 	println!("{}", r2);
// 	println!("{}", r3);
}

fn printRect<T>( r: &Rect<T> ) where T: fmt::Display {
	println!("{}", r);
}
