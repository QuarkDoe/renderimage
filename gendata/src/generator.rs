// extern crate noise;

use std::fmt::Display;

use rand::{XorShiftRng,SeedableRng,Rng,Rand,random};
use num::*;
use num::traits::FloatConst;

use super::gen_type as GenType;

use geom::rect::*;
use geom::vector::*;
use misc::*;

use std::ops::Mul;

fn random_single_v<T>() -> Vector<T> where T:Float + FloatConst + Rand {
	let pi2 = T::PI() + T::PI();
	let t = pi2 * random::<T>();
	Vector::<T>::new ( t.cos(), t.sin() )
}

struct Generated<T> {
	t: GenType,
	r: Rect<T>,
	v: Vector<T>
}

pub struct Generator<T> {
	size: ( T, T ),
	max_item_size: ( T, T ),
	tile_size: T,
	item_number: usize,
	seed : [u32; 4],
	data : Vec<Generated<T>>,
	generated: bool,
}

impl<T> Generator<T>
	where T:Float + FloatConst + Rand + Display + RoundM<T>
{

	pub fn new() -> Generator<T> {
		Generator::<T>{
			size: (
				T::from(100).unwrap(),
				T::from(100).unwrap()
			),
			max_item_size: (
				T::from(30).unwrap(),
				T::from(30).unwrap()
			),
			tile_size: T::from(10).unwrap(),
			item_number: 10,
			data: vec![],
			seed : [1, 2, 3, 4],
			generated: false 
		}
	}
	
	pub fn set_size( &mut self, w: T, h:T ){
		self.size = ( w.abs(), h.abs() );
		self.generated = false;
	}
	
	pub fn get_size( &self ) -> ( T, T ) {
		self.size
	}

	pub fn set_max_item_size( &mut self, w: T, h:T ){
		self.max_item_size = ( w.abs(), h.abs() );
		self.generated = false;
	}
	
	pub fn get_max_item_size( &self ) -> ( T, T ) {
		self.size
	}

	pub fn set_tile_size( &mut self, s: T ) {
		self.tile_size = s.abs();
		self.generated = false;
	}

	pub fn get_tile_size( &self ) -> T {
		self.tile_size
	}

	pub fn set_item_number( &mut self, n: usize ) {
		self.item_number = n;
		self.generated = false;
	}

	pub fn get_item_number( &self ) -> usize {
		self.item_number
	}

	
	pub fn generate( &mut self ) -> bool {
		self.data.clear();

		// константы
		let _1 = T::one();
		let _2 = _1 + _1;
		let WH_MAX_REL = T::from(1.5).unwrap();
		let WH_MIN_REL = T::from(0.66).unwrap();
// 		let WH_MAX_REL = T::from(1.25).unwrap();
// 		let WH_MIN_REL = T::from(0.8).unwrap();

		let ts = self.tile_size;
		let ( miw, mih ) = self.max_item_size;
// 		let msq = miw * mih / _2;
		let msq = miw * mih * T::from(0.75).unwrap();

		let _pi2 = T::PI() + T::PI();
		let width = self.size.0 / _2;
		let height = self.size.1 / _2;
		
		// вектор подходящих площадей.
		let mut sq = Vec::<( usize, T )>::new();
		
		let mut rng = XorShiftRng::from_seed(self.seed);
		for idx in 0..self.item_number {
			let ( x, y ) = {
				let t = _pi2 * rng.gen::<T>();
				let u = rng.gen::<T>() + rng.gen::<T>();
				let r = if u > _1 { _2 - u } else { u };
				( 
					( width * r * t.cos() / _2 + width ).roundm( ts ),
					( height * r * t.sin() / _2 + height ).roundm( ts )
				)
			};
			let ( w, h ) = {
				(
					( rng.gen::<T>() * miw + ts ).roundm( ts ),
					( rng.gen::<T>() * mih + ts ).roundm( ts )
				)
			};
			self.data.push( Generated{
				t: GenType::Common,
				r: Rect::<T>::new( x, y, w, h ),
				v: Vector::<T>::zero(),
			} ) ;
			
			let rel = w / h;
			let s = w * h;
			if rel > WH_MIN_REL && rel < WH_MAX_REL && s > msq { sq.push( ( idx, s ) ); };
		}
		
// 		let sq_len = sq.len();
// 		let t = sq_len * 5 / 10;
// 		println!( "{} {}", sq_len, t );
// 		sq.sort_by(|ref a, ref b| a.1.partial_cmp(&b.1).unwrap() );
		for n in sq.iter() {
			self.data[n.0].t = GenType::Main;
		}
// 		for i in t..sq_len {
// 			self.data[sq[i].0].t = GenType::Main;
// 		}
// 		if sq_len > 0 {
// 			let mean = match sq_len % 2 {
// 				0 => ( sq[ sq_len / 2 ].1 + sq[ ( sq_len - 2 ) / 2 ].1 ) / _2,
// 				_ => sq[ sq_len / 2 ].1,
// 			};
// 		}
		
		
		self.calculate_vectors();
		self.generated = true;
		self.generated
	}
	
	fn calculate_vectors( &mut self ) -> bool {
	
		let mut d = &mut self.data;
		let len = d.len();
		
		for i in 0..len {
			let mut v = Vector::<T>::zero();
			for j in 0..len {
				if i != j {
					let ri = &d[i].r;
					let rj = &d[j].r;
					if ri.is_intersect(&rj) {
						let vij = ri.center().vector_from(rj.center());
						v += if vij.is_zero() { random_single_v() } else { vij.norm() }
					}
				}
			}
// 			d[i].v = v;
// 			d[i].v = v.norm().mul( self.tile_size * T::SQRT_2() ).round_n( self.tile_size );
// 			d[i].v = v.norm().mul( self.tile_size * T::SQRT_2() );
			d[i].v = v.norm().mul( self.tile_size );
		}
		true
	}

	pub fn distribute( &mut self ) -> bool {
// 		for ref mut s in &mut self.data {
		for ( i, ref mut s ) in self.data.iter_mut().enumerate() { 
// 			s.r.move_v( s.v.norm().mul( self.tile_size * T::SQRT_2() ).round_n( self.tile_size ) );
			let v = s.v.round_n( self.tile_size );
			println!( "{} {} {}", i, s.v, v );
			s.r.move_v( v );
			s.v = Vector::<T>::zero();
		}
		self.calculate_vectors();
		true
	}
	
	pub fn get_generated_len( &self ) -> usize{
		self.data.len()
	}

	pub fn get_generated_type( &self, idx:usize ) -> GenType{
		self.data[idx].t
	}

	pub fn get_generated_rect<'a>( &'a self, idx:usize ) -> &'a Rect<T>{
		&self.data[idx].r
	}

	pub fn get_generated_vector<'a>( &'a self, idx:usize ) -> &'a Vector<T>{
		&self.data[idx].v
	}
	
	pub fn is_generated( &self ) -> bool {
		self.generated
	}
}

// fn get_random_point_in_ellipse( rng:&mut Rng, width:f64, height:f64 ) -> ( f64, f64 ){
// 	let t = 2. * PI * rng.next_f64();
// 	let u = rng.next_f64() + rng.next_f64();
// 	let r = if u > 1. { 2. - u } else { u };
// 	( width * r * t.cos() / 2., height * r * t.sin() / 2. )
// }

// fn distribute(data: &mut Vec<Generated>){
// }

// function getRandomPointInEllipse(ellipse_width, ellipse_height)
//   local t = 2*math.pi*math.random()
//   local u = math.random()+math.random()
//   local r = nil
//   if u > 1 then r = 2-u else r = u end
//   return roundm(ellipse_width*r*math.cos(t)/2, tile_size), 
//          roundm(ellipse_height*r*math.sin(t)/2, tile_size)
// end
// fn center( b: &GenBounds ) -> GenCoord {
// 	GenCoord { x: b.x + b.width as / 2,  }
// }

/*
trait RoundByM<T:Float,M:Num+ToPrimitive>{
	fn roundm(n:T, m:M) -> T;
// 	fn roundm(n:T, m:M) -> T{
// 		let d = T::from(m).unwrap();
// 		((n + d - T::one())/d).floor() * d
// 	}
}
*/
// trait RoundByM<M>{
// 	fn roundm(&self, m:M) -> Self;
// }
// 
// impl <M:Num+ToPrimitive> RoundByM<M> for f64{
// 	fn roundm(&self, m:M) -> f64{
// 		let d = m.to_f64().unwrap();
// 		((self + d - f64::one())/d).floor() * d
// 	}
// }
// 
// impl <M:Num+ToPrimitive> RoundByM<M> for f32{
// 	fn roundm(&self, m:M) -> f32{
// 		let d = m.to_f32().unwrap();
// 		((self + d - f32::one())/d).floor() * d
// 	}
// }

// fn roundm<T:Float>(n:T, m:T) -> T{
// 	((n + m - T::one())/m).floor() * m
// }
// fn roundm<T:Float,M:Num+ToPrimitive>(n:T, m:M) -> T{

// 					(roundm( width * r * t.cos() / 2. , tile_size ) + width ) as i32,
// 					(roundm( height * r * t.sin() / 2. , tile_size ) + height ) as i32

// fn roundm<T,M>( n:T, m:M ) -> M where T:Float, M:Num + NumCast {
// 	let d = T::from(m).unwrap();
// 	M::from(((n + d - T::one())/d).floor() * d).unwrap()
// }

// fn roundm_v<T,M>( v:Vector<T>, m:T ) -> Vector<T> where T:Float {
// 	Vector::<T>::new( roundm( v.dx, m ), roundm( v.dy, m ) )
// }

// fn random_v<T>( r:T ) -> Vector<T> where T:Float + FloatConst + Rand {
// 	let pi2 = T::PI() + T::PI();
// 	let t = pi2 * random::<T>();
// 	Vector::<T>::new ( r*t.cos(), r*t.sin() )
// }


