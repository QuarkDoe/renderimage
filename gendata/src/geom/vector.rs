use num::*;
use geom::point::*;
use std::fmt;
use std::ops;

#[repr(C)]
#[derive(Eq, PartialEq, Clone, Hash, Debug, Copy)]
pub struct Vector<T> {
	/// First component of the point.
	pub dx: T,
	/// Second component of the point.
	pub dy: T
}

impl<T> Vector<T>  where T: Float {

	/// Returns a new `Point` with the given `x` and `y` coordinates.
	pub fn new( dx: T, dy: T ) -> Vector<T> {
		Vector::<T> { dx: dx, dy: dy }
	}
	
	pub fn to_point(&self) -> Point<T> {
		Point::<T>::new( self.dx, self.dy )
	}
	
	pub fn len(&self) -> T {
		( self.dx * self.dx + self.dy * self.dy ).sqrt()
	}
	
	pub fn norm(&self) -> Vector<T> {
		let len = self.len();
		if len > T::zero() { Vector::<T> { dx: self.dx/len, dy: self.dy/len } }
		else { Vector::<T>::zero() }
	}
	
	pub fn round_n(&self, n:T) -> Vector<T> {
		let d = n.abs();
// 		let _1 = T::one();
// 		let ndx = ( ( self.dx.abs() + d - _1 )/d ).floor() * d * self.dx.signum();
// 		let ndy = ( ( self.dy.abs() + d - _1 )/d ).floor() * d * self.dy.signum();
		let ndx = ( ( self.dx )/d ).round() * d;
		let ndy = ( ( self.dy )/d ).round() * d;
		Vector::<T>::new( ndx, ndy )
	}
	
}

impl<T> Zero for Vector<T>  where T: Float {
	
	#[inline]
	fn zero() -> Vector<T> {
		Vector::<T>::new(T::zero(), T::zero())
	}
	
	#[inline]
	fn is_zero(&self) -> bool {
		self.dx.is_zero() && self.dy.is_zero()
	}
}

impl<T> ops::Add for Vector<T> where T: Float {
	type Output = Vector<T>;
	
	fn add(self, other: Vector<T>) -> Vector<T> {
		Vector::<T> {
			dx: self.dx + other.dx,
			dy: self.dy + other.dy,
		}
	}
}

impl<T> ops::AddAssign for Vector<T> where T: Float {
	fn add_assign(&mut self, other: Vector<T>) {
		self.dx = self.dx + other.dx;
		self.dy = self.dy + other.dy;
	}
}

impl<T> ops::Sub for Vector<T> where T: Float {
	type Output = Vector<T>;
	
	fn sub(self, other: Vector<T>) -> Vector<T> {
		Vector::<T> {
			dx: self.dx - other.dx,
			dy: self.dy - other.dy,
		}
	}
}

impl<T> ops::SubAssign for Vector<T> where T: Float {
	fn sub_assign(&mut self, other: Vector<T>) {
		self.dx = self.dx - other.dx;
		self.dy = self.dy - other.dy;
	}
}

impl<T> ops::Mul<T> for Vector<T> where T: Float {
	type Output = Vector<T>;

	fn mul(self, r: T) -> Vector<T> {
		Vector::<T>::new( self.dx * r, self.dy * r )
	}
}

impl<T> ops::MulAssign<T> for Vector<T> where T: Float {
	fn mul_assign( &mut self, r:T ){
		self.dx = self.dx * r;
		self.dy = self.dy * r;
	}
}

impl<T> ops::Neg for Vector<T> where T: Float {
	type Output = Vector<T>;
	
	fn neg(self) -> Vector<T>{
		Vector::<T> {
			dx: -self.dx,
			dy: -self.dy,
		}
    }
}

impl<T> fmt::Display for Vector<T> where T: fmt::Display {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "v({}, {})", self.dx, self.dy)
	}
}

