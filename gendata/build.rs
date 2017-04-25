extern crate cheddar;

fn main() {
	println!("Generate");
	cheddar::Builder::c99()
	.expect("could not read manifest")
	.name("gendata.h")
	.insert_code("// version 1.0\n#include <stddef.h>" )
	.namespace_enums( false )
	.run_build();
}

