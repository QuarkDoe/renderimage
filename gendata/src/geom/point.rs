use num::*;
use geom::rect::*;
use geom::vector::*;
use misc::*;
use std::fmt;

#[repr(C)]
#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct Point<N> {
	/// First component of the point.
	pub x: N,
	/// Second component of the point.
	pub y: N
}

impl<T> Point<T> where T: Float {

	/// Returns a new `Point` with the given `x` and `y` coordinates.
	pub fn new( x: T, y: T ) -> Point<T> {
		Point::<T> { x: x, y: y }
	}

	pub fn zero() -> Point<T> {
		Point::<T>::new( T::zero(), T::zero() )
	}
	
	#[inline]
	pub fn is_zero(&self) -> bool {
		self.x.is_zero() && self.y.is_zero()
	}

	/// convert point to vector
	pub fn to_vector(&self) -> Vector<T> {
		Vector::<T>::new( self.x, self.y )
	}
	
	/// move point by vector
	pub fn move_v( &mut self, v: Vector<T> ) -> &mut Self {
		self.x = self.x + v.dx;
		self.y = self.y + v.dy;
		self
	}
	
	/// move point by vector
	pub fn move_vm( &mut self, v: Vector<T>, m:T ) -> &mut Self {
		self.x = ( self.x + v.dx ).roundm(m);
		self.y = ( self.y + v.dy ).roundm(m);
		self
	}
	
	/// Returns a new `Rect` with the top-left point being the value of `self`
	/// and the bottom-right point being the value of `other`.
	pub fn rect( self, other: Point<T>) -> Rect<T> {
		Rect::from_points( self, other )
	}
	
	pub fn vector_to( self, other: Point<T> ) -> Vector<T> {
		Vector::<T>::new( other.x - self.x , other.y - self.y )
	}

	pub fn vector_from( self, other: Point<T> ) -> Vector<T> {
		Vector::<T>::new( self.x - other.x , self.y - other.y )
	}
}

impl<T> Default for Point<T> where T: Default {
	/// Returns a new `Point` with the `x` and `y` coordinates being that of the default value of `T`.
	fn default() -> Point<T> {
		Point::<T> { x: T::default(), y: T::default() }
	}
}

// impl Display for Rect<T> where T: Float + Default + Copy + Clone + Display {
impl<T> fmt::Display for Point<T> where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "p({}, {})", self.x, self.y)
    }
}
