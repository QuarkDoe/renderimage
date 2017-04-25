//
//
//


pub mod point;
pub mod vector;
pub mod poly;
pub mod rect;

#[cfg(test)]
mod tests {

use geom::point::*;
use geom::rect::*;
use geom::vector::*;

// use std::io::*;
// use std::io::{self,Write};
use std::fmt::Write;

#[test]
fn intersection() {
	let r1 = Rect::from_points(
		Point::new( 1., 3. ),
		Point::new( 2., 0. )
	);
	let r2 = Rect::from_points(
		Point::new( 0., 2. ),
		Point::new( 3., 1. )
	);
	let r3 = r1.get_intersect(&r2).unwrap();
}

#[test]
fn vectors() {
	let r1 = Rect::from_points(
		Point::new( 1.5, 3. ),
		Point::new( 2.5, 0. )
	);
	let r2 = Rect::from_points(
		Point::new( 0., 2. ),
		Point::new( 3., 1. )
	);
	let v1 =  r1.center().vector_to(r2.center());
	let v2 =  r2.center().vector_to(r1.center());
	let v3 = v1 + v2;
	let v3n = v3.norm();
	assert!( v3n.len().is_zero() )
}

}
