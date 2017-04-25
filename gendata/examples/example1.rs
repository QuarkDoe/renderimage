extern crate gendata;

use gendata::*;

fn main() {
	let mut gen = generator::Generator::<f32>::new();
	gen.set_size( 1000.0f32, 1000.0f32 );
	gen.generate();
	gen.distribute();
	println!("Hello, world!");
}
