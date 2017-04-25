use num::*;
use geom::point::*;
use geom::vector::*;
use std::fmt;

/// A generic rectangle structure.
#[repr(C)]
#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct Rect<T> {
	pub p1: Point<T>,
	pub p2: Point<T>
}

impl<T> Rect<T> where T: Float {
	/// Returns a new rectangle with the supplied
	/// upper left corner at (x, y) and with the given width and height.
	pub fn new( x: T, y: T, width: T, height: T ) -> Rect<T> {
		Rect::<T> {
			p1: Point::new( x, y ),
			p2: Point::new( x + width.abs(), y - height.abs() )
		}
	}

	/// Returns a new rectangle with the given top-left and bottom-right points.
	pub fn from_points( p1: Point<T>, p2: Point<T> ) -> Rect<T> {
		Rect { p1: p1, p2: p2 }
	}

	/// Retruns a center point of rectangle.
	pub fn center(&self) -> Point<T> {
		Point::<T>::new( ( self.p1.x + self.p2.x )/T::from(2.).unwrap(), ( self.p1.y + self.p2.y )/T::from(2.).unwrap() )	
	}

	pub fn get_center( &self, r: &mut Point<T> ) {
		r.x = ( self.p1.x + self.p2.x )/T::from(2.).unwrap();
		r.y = ( self.p1.y + self.p2.y )/T::from(2.).unwrap();
	}
	
	/// Returns the width of the rectangle.
	pub fn width(&self) -> T {
		( self.p1.x - self.p2.x ).abs()
	}

	/// Returns the height of the rectangle.
	pub fn height(&self) -> T {
		( self.p1.y - self.p2.y ).abs()
	}

	/// Returns a copy of the top-left point of the rectangle.
	pub fn top_left(&self) -> Point<T> {
		Point::new( self.left(), self.top() )
	}

	pub fn top_right(&self) -> Point<T> {
		Point::new( self.right(), self.top() )
	}

	/// Returns a copy of the bottom-right point of the rectangle.
	pub fn bottom_right(&self) -> Point<T> {
		Point::new( self.right(), self.bottom() )
	}

	pub fn bottom_left(&self) -> Point<T> {
		Point::new( self.left(), self.bottom() )
	}

	/// Returns the `y` coordinate of the top side of the rectangle.
	pub fn top(&self) -> T {
		self.p1.y.max( self.p2.y )
	}

	/// Returns the `x` coordinate of the left side of the rectangle.
	pub fn left(&self) -> T {
		self.p1.x.min( self.p2.x )
	}

	/// Returns the `y` coordinate of the bottom side of the rectangle.
	pub fn bottom(&self) -> T {
		self.p1.y.min( self.p2.y )
	}

	/// Returns the `x` coordinate of the right side of the rectangle.
	pub fn right(&self) -> T {
		self.p1.x.max( self.p2.x )
	}

	/// Returns the area of the rectangle.
	pub fn area(&self) -> T {
		self.width() * self.height()
	}

	/// move point by vector
	pub fn move_v( &mut self, v: Vector<T> ) -> &mut Self {
		self.p1.move_v( v );
		self.p2.move_v( v );
		self
	}

	/// move point by vector
	pub fn move_vm( &mut self, v: Vector<T>, m:T ) -> &mut Self {
		self.p1.move_vm( v, m );
		self.p2.move_vm( v, m );
		self
	}

	/// Returns `true` if the given point lies within the bounds of the rectangle,
	/// and `false` otherwise.
	pub fn contains(&self, point: Point<T>) -> bool {
		self.left() <= point.x && point.x <= self.right() &&
		self.top() <= point.y && point.y <= self.bottom()
	}

	/// If `self` and `other` intersect, then the intersection of the two rectangles
	/// is returned as a new rectangle, otherwise `None` is returned.
	pub fn get_intersect(&self, other: &Rect<T>) -> Option<Rect<T>> {
		let left = self.left().max( other.left() );
		let right = self.right().min( other.right() );
		let top = self.top().min( other.top() );
		let bottom = self.bottom().max( other.bottom() );

		if left <= right && top >= bottom {
// 		if left < right && top > bottom {
			Some( Point::new( left, top ).rect( Point::new( right, bottom ) ) )
		} else {
			None
		}
	}

	/// Return `true` if `self` and `other` intersect, otherwise `false` is returned.
	pub fn is_intersect(&self, other: &Rect<T>) -> bool {
		let left = self.left().max( other.left() );
		let right = self.right().min( other.right() );
		let bottom = self.bottom().max( other.bottom() );
		let top = self.top().min( other.top() );

		left <= right && top >= bottom
	}

	/// Return `true` if `self` and `other` intersect, otherwise `false` is returned.
	pub fn is_intersect_strict(&self, other: &Rect<T>) -> bool {
		let left = self.left().max( other.left() );
		let right = self.right().min( other.right() );
		let bottom = self.bottom().max( other.bottom() );
		let top = self.top().min( other.top() );

		left < right && top > bottom
	}

}

impl<T> fmt::Display for Rect<T> where T: fmt::Display {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "r({}, {})", self.p1, self.p2)
	}
}

impl<T> IntoIterator for Rect<T> where T: Float {
	type Item = Point<T>;
	type IntoIter = ::std::vec::IntoIter<Point<T>>;
	fn into_iter(self) -> Self::IntoIter {
		( vec![
			self.top_left(),
			self.top_right(),
			self.bottom_right(),
			self.bottom_left(),
		] ).into_iter()
// 		Iter::<T> { cur: self.top_left(), cur_back: self.bottom_right(), rect: self }
	}
}

/*
impl<T> Rect<T> where T: PrimInt + Signed + Default + Copy + Clone {
	/// Returns a new rectangle with the supplied
	/// upper left corner at (x, y) and with the given width and height.
	pub fn new( x: T, y: T, width: T, height: T ) -> Rect<T> {
		Rect::<T> {
			p1: Point::new( x, y ),
			p2: Point::new( x + width.abs() - T::one(), y - height.abs() + T::one() )
		}
	}

	/// Returns a new rectangle with the given top-left and bottom-right points.
	pub fn from_points( p1: Point<T>, p2: Point<T> ) -> Rect<T> {
		Rect { p1: p1, p2: p2 }
	}

	/// Retruns a center point of rectangle.
	pub fn center(&self) -> Point<T> {
		Point::<T>::new( ( self.p1.x + self.p2.x )/T::from(2).unwrap(), ( self.p1.y + self.p2.y )/T::from(2).unwrap() )	
	}
	
	/// Returns the width of the rectangle.
	pub fn width(&self) -> T {
		( self.p1.x - self.p2.x ).abs() + T::one()
	}

	/// Returns the height of the rectangle.
	pub fn height(&self) -> T {
		( self.p1.y - self.p2.y ).abs() + T::one()
	}

	/// Returns a copy of the top-left point of the rectangle.
	pub fn top_left(&self) -> Point<T> {
		Point::new( self.left(), self.top() )
	}

	/// Returns a copy of the bottom-right point of the rectangle.
	pub fn bottom_right(&self) -> Point<T> {
		Point::new( self.right(), self.bottom() )
	}

	/// Returns the `y` coordinate of the top side of the rectangle.
	pub fn top(&self) -> T {
		self.p1.y.max( self.p2.y )
	}

	/// Returns the `x` coordinate of the left side of the rectangle.
	pub fn left(&self) -> T {
		self.p1.x.min( self.p2.x )
	}

	/// Returns the `y` coordinate of the bottom side of the rectangle.
	pub fn bottom(&self) -> T {
		self.p1.y.min( self.p2.y )
	}

	/// Returns the `x` coordinate of the right side of the rectangle.
	pub fn right(&self) -> T {
		self.p1.x.max( self.p2.x )
	}

	/// Returns the area of the rectangle.
	pub fn area(&self) -> T {
		self.width() * self.height()
	}

	/// Returns `true` if the given point lies within the bounds of the rectangle,
	/// and `false` otherwise.
	pub fn contains(&self, point: Point<T>) -> bool {
		self.left() <= point.x && point.x <= self.right() &&
		self.top() <= point.y && point.y <= self.bottom()
	}

	/// If `self` and `other` intersect, then the intersection of the two rectangles
	/// is returned as a new rectangle, otherwise `None` is returned.
	pub fn get_intersect(&self, other: &Rect<T>) -> Option<Rect<T>> {
		let left = self.left().max( other.left() );
		let right = self.right().min( other.right() );
		let bottom = self.bottom().max( other.bottom() );
		let top = self.top().min( other.top() );

		if left <= right && top <= bottom {
			Some( Point::new( left, top ).rect( Point::new( right, bottom ) ) )
		} else {
			None
		}
	}

	/// If `self` and `other` intersect, then the intersection of the two rectangles
	/// is returned as a new rectangle, otherwise `None` is returned.
	pub fn is_intersect(&self, other: &Rect<T>) -> bool {
		let left = self.left().max( other.left() );
		let right = self.right().min( other.right() );
		let bottom = self.bottom().max( other.bottom() );
		let top = self.top().min( other.top() );

		left <= right && top <= bottom
	}

}
*/
