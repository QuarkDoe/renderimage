use num::*;

// pub fn roundm<T,M>( n:T, m:M ) -> M where T:Float, M:Num + NumCast {
// 	let d = T::from(m).unwrap();
// 	M::from(((n + d - T::one())/d).floor() * d).unwrap()
// }

pub trait RoundM<M> {
	fn roundm( &self, m:M ) -> M;
}

impl<T,M> RoundM<M> for T where T:Float, M:Num + NumCast {

	fn roundm( &self, m:M ) -> M {
		let d = T::from(m).unwrap();
		M::from(((*self + d - T::one())/d).floor() * d).unwrap()
	}

}
