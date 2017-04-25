extern crate libc;
extern crate num;
extern crate rand;

use libc::size_t;

pub mod generator;
pub mod geom;
pub mod misc;

unsafe fn as_mut_ref<'a, T>( p: *mut T ) -> Option<&'a mut T> {
	if p.is_null() {
		None
	} else {
		Some(&mut *p)
	}
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum gen_type {
    Common,
    Main,
    Spawn,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct gen_point_f32{
	pub x: f32,
	pub y: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct gen_vector_f32{
	pub dx: f32,
	pub dy: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct gen_rect_f32{
	pub p1: gen_point_f32,
	pub p2: gen_point_f32,
}

#[repr(C)]
pub struct gen_struct_f32 {
}

#[no_mangle]
pub extern "C" fn generator_f32_new() -> *mut gen_struct_f32 {
	Box::into_raw( Box::new( generator::Generator::<f32>::new() ) ) as *mut gen_struct_f32
}

#[no_mangle]
pub extern "C" fn generator_f32_delete( g_ptr: *mut gen_struct_f32 ) {
	println!("generator_f32_delete");
	unsafe {
		Box::from_raw(g_ptr as *mut generator::Generator<f32>);
	}
}

#[no_mangle]
pub extern "C" fn generator_f32_generate( g_ptr: *mut gen_struct_f32 ) -> bool {
	unsafe{
		as_mut_ref( g_ptr as *mut generator::Generator<f32> ).expect( "g_ptr: Null pointer!!!" ).generate()
	}
}

#[no_mangle]
pub extern "C" fn generator_f32_distribute( g_ptr: *mut gen_struct_f32 ) -> bool {
	unsafe{
		as_mut_ref( g_ptr as *mut generator::Generator<f32> ).expect( "g_ptr: Null pointer!!!" ).distribute()
	}
}

#[no_mangle]
pub extern "C" fn generator_f32_isgenerated( g_ptr: *const gen_struct_f32 ) -> bool {
	unsafe{
		( g_ptr as *mut generator::Generator<f32> ).as_ref().expect( "g_ptr: Null pointer!!!" ).is_generated()
	}
}

#[no_mangle]
pub extern "C" fn generator_f32_set_size( g_ptr: *mut gen_struct_f32, width: f32, height: f32 ) {
	unsafe{
		let gen = as_mut_ref( g_ptr as *mut generator::Generator<f32> ).expect( "g_ptr: Null pointer!!!" );
		gen.set_size( width, height );
	}
}

#[no_mangle]
pub extern "C" fn generator_f32_get_size( g_ptr: *mut gen_struct_f32, width: *mut f32, height: *mut f32 ) {
	unsafe{
		let gen = as_mut_ref( g_ptr as *mut generator::Generator<f32> ).expect( "g_ptr: Null pointer!!!" );
		let rw = as_mut_ref( width ).expect( "width: Null pointer!!!" );
		let rh = as_mut_ref( height ).expect( "height: Null pointer!!!" );
		let ( w, h ) = gen.get_size();
		*rw = w;
		*rh = h;
	}
}

#[no_mangle]
pub extern "C" fn generator_f32_set_max_item_size( g_ptr: *mut gen_struct_f32, width: f32, height: f32 ) {
	unsafe{
		let gen = as_mut_ref( g_ptr as *mut generator::Generator<f32> ).expect( "g_ptr: Null pointer!!!" );
		gen.set_max_item_size( width, height );
	}
}

#[no_mangle]
pub extern "C" fn generator_f32_get_max_item_size( g_ptr: *mut gen_struct_f32, width: *mut f32, height: *mut f32 ) {
	unsafe{
		let gen = as_mut_ref( g_ptr as *mut generator::Generator<f32> ).expect( "g_ptr: Null pointer!!!" );
		let rw = as_mut_ref( width ).expect( "width: Null pointer!!!" );
		let rh = as_mut_ref( height ).expect( "height: Null pointer!!!" );
		let ( w, h ) = gen.get_max_item_size();
		*rw = w;
		*rh = h;
	}
}

#[no_mangle]
pub extern "C" fn generator_f32_set_tile_size( g_ptr: *mut gen_struct_f32, tile_size: f32 ) {
	unsafe{
		let gen = as_mut_ref( g_ptr as *mut generator::Generator<f32> ).expect( "g_ptr: Null pointer!!!" );
		gen.set_tile_size( tile_size );
	}
}

#[no_mangle]
pub extern "C" fn generator_f32_get_tile_size( g_ptr: *mut gen_struct_f32 ) -> f32 {
	unsafe{
		as_mut_ref( g_ptr as *mut generator::Generator<f32> ).expect( "g_ptr: Null pointer!!!" )
		.get_tile_size()
	}
}

#[no_mangle]
pub extern "C" fn generator_f32_set_item_number( g_ptr: *mut gen_struct_f32, item_number: size_t ) {
	unsafe{
		let gen = as_mut_ref( g_ptr as *mut generator::Generator<f32> ).expect( "g_ptr: Null pointer!!!" );
		gen.set_item_number( item_number );
	}
}

#[no_mangle]
pub extern "C" fn generator_f32_get_item_number( g_ptr: *mut gen_struct_f32 ) -> size_t {
	unsafe{
		as_mut_ref( g_ptr as *mut generator::Generator<f32> ).expect( "g_ptr: Null pointer!!!" )
		.get_item_number()
	}
}

#[no_mangle]
pub extern "C" fn generator_f32_get_generated_len( g_ptr: *const gen_struct_f32 ) -> size_t {
	unsafe{
		( g_ptr as *mut generator::Generator<f32> ).as_ref().expect( "g_ptr: Null pointer!!!" )
		.get_generated_len() as size_t
	}
}

#[no_mangle]
pub extern "C" fn generator_f32_get_generated_type( g_ptr: *const gen_struct_f32, idx: size_t ) -> gen_type {
	unsafe{
		( g_ptr as *mut generator::Generator<f32> ).as_ref().expect( "g_ptr: Null pointer!!!" )
		.get_generated_type( idx as usize )
	}
}

#[no_mangle]
pub extern "C" fn generator_f32_get_generated_rect( g_ptr: *const gen_struct_f32, idx: size_t ) -> *const gen_rect_f32 {
	unsafe{
		use geom::rect::Rect;
		let r = ( g_ptr as *mut generator::Generator<f32> ).as_ref().expect( "g_ptr: Null pointer!!!" )
		.get_generated_rect( idx as usize ) as *const Rect<f32>;
		std::mem::transmute::< *const Rect<f32>, *const gen_rect_f32 >( r )
	}
}

#[no_mangle]
pub extern "C" fn generator_f32_get_generated_rect_center( g_ptr: *const gen_struct_f32, idx: size_t, r_ptr: *const gen_point_f32 ) {
	unsafe{
		use geom::point::Point;
		use generator::Generator;
		let rect = ( g_ptr as *mut Generator<f32> ).as_ref().expect( "g_ptr: Null pointer!!!" ).get_generated_rect( idx as usize );
		let res = as_mut_ref( r_ptr as *mut Point<f32> ).expect( "g_ptr: Null pointer!!!" );
		rect.get_center( res );
	}
}

// #[no_mangle]
// pub extern "C" fn generator_f32_get_generated_center( g_ptr: *const gen_struct_f32, idx: size_t ) -> gen_point_f32 {
// 	unsafe{
// 		use geom::point::Point;
// 		let r = ( g_ptr as *mut generator::Generator<f32> ).as_ref().expect( "g_ptr: Null pointer!!!" )
// 		.get_generated_rect( idx as usize );
// 		let p = r.center();
// 		std::mem::transmute::< Point<f32>, gen_point_f32 >( p )
// 	}
// }

#[no_mangle]
pub extern "C" fn generator_f32_get_generated_vector( g_ptr: *const gen_struct_f32, idx: size_t ) -> *const gen_vector_f32 {
	unsafe{
		use geom::vector::Vector;
		let v = ( g_ptr as *mut generator::Generator<f32> ).as_ref().expect( "g_ptr: Null pointer!!!" )
		.get_generated_vector( idx as usize ) as *const Vector<f32>;
		std::mem::transmute::< *const Vector<f32>, *const gen_vector_f32>( v )
	}
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct gen_point_f64{
	pub x: f64,
	pub y: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct gen_vector_f64{
	pub dx: f64,
	pub dy: f64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct gen_rect_f64{
	pub p1: gen_point_f64,
	pub p2: gen_point_f64,
}

#[repr(C)]
pub struct gen_struct_f64 {
}

#[no_mangle]
pub extern "C" fn generator_f64_new() -> *mut gen_struct_f64 {
	Box::into_raw( Box::new( generator::Generator::<f64>::new() ) ) as *mut gen_struct_f64
}

#[no_mangle]
pub extern "C" fn generator_f64_delete( g_ptr: *mut gen_struct_f64 ) {
	unsafe {
		Box::from_raw(g_ptr as *mut generator::Generator<f64>);
	}
}

#[no_mangle]
pub extern "C" fn generator_f64_generate( g_ptr: *mut gen_struct_f64 ) -> bool {
	unsafe{
		as_mut_ref( g_ptr as *mut generator::Generator<f64> ).expect( "g_ptr: Null pointer!!!" ).generate()
	}
}

#[no_mangle]
pub extern "C" fn generator_f64_distribute( g_ptr: *mut gen_struct_f64 ) -> bool {
	unsafe{
		as_mut_ref( g_ptr as *mut generator::Generator<f64> ).expect( "g_ptr: Null pointer!!!" ).distribute()
	}
}

#[no_mangle]
pub extern "C" fn generator_f64_isgenerated( g_ptr: *const gen_struct_f64 ) -> bool {
	unsafe{
		( g_ptr as *mut generator::Generator<f64> ).as_ref().expect( "g_ptr: Null pointer!!!" ).is_generated()
	}
}

#[no_mangle]
pub extern "C" fn generator_f64_set_size( g_ptr: *mut gen_struct_f64, width: f64, height: f64 ) {
	unsafe{
		let gen = as_mut_ref( g_ptr as *mut generator::Generator<f64> ).expect( "g_ptr: Null pointer!!!" );
		gen.set_size( width, height );
	}
}

#[no_mangle]
pub extern "C" fn generator_f64_get_size( g_ptr: *mut gen_struct_f64, width: *mut f64, height: *mut f64 ) {
	unsafe{
		let gen = as_mut_ref( g_ptr as *mut generator::Generator<f64> ).expect( "g_ptr: Null pointer!!!" );
		let rw = as_mut_ref( width ).expect( "width: Null pointer!!!" );
		let rh = as_mut_ref( height ).expect( "height: Null pointer!!!" );
		let ( w, h ) = gen.get_size();
		*rw = w;
		*rh = h;
	}
}

#[no_mangle]
pub extern "C" fn generator_f64_set_max_item_size( g_ptr: *mut gen_struct_f64, width: f64, height: f64 ) {
	unsafe{
		let gen = as_mut_ref( g_ptr as *mut generator::Generator<f64> ).expect( "g_ptr: Null pointer!!!" );
		gen.set_max_item_size( width, height );
	}
}

#[no_mangle]
pub extern "C" fn generator_f64_get_max_item_size( g_ptr: *mut gen_struct_f64, width: *mut f64, height: *mut f64 ) {
	unsafe{
		let gen = as_mut_ref( g_ptr as *mut generator::Generator<f64> ).expect( "g_ptr: Null pointer!!!" );
		let rw = as_mut_ref( width ).expect( "width: Null pointer!!!" );
		let rh = as_mut_ref( height ).expect( "height: Null pointer!!!" );
		let ( w, h ) = gen.get_max_item_size();
		*rw = w;
		*rh = h;
	}
}

#[no_mangle]
pub extern "C" fn generator_f64_set_tile_size( g_ptr: *mut gen_struct_f64, tile_size: f64 ) {
	unsafe{
		let gen = as_mut_ref( g_ptr as *mut generator::Generator<f64> ).expect( "g_ptr: Null pointer!!!" );
		gen.set_tile_size( tile_size );
	}
}

#[no_mangle]
pub extern "C" fn generator_f64_get_tile_size( g_ptr: *mut gen_struct_f64 ) -> f64 {
	unsafe{
		as_mut_ref( g_ptr as *mut generator::Generator<f64> ).expect( "g_ptr: Null pointer!!!" )
		.get_tile_size()
	}
}

#[no_mangle]
pub extern "C" fn generator_f64_set_item_number( g_ptr: *mut gen_struct_f64, item_number: size_t ) {
	unsafe{
		let gen = as_mut_ref( g_ptr as *mut generator::Generator<f64> ).expect( "g_ptr: Null pointer!!!" );
		gen.set_item_number( item_number );
	}
}

#[no_mangle]
pub extern "C" fn generator_f64_get_item_number( g_ptr: *mut gen_struct_f64 ) -> size_t {
	unsafe{
		as_mut_ref( g_ptr as *mut generator::Generator<f64> ).expect( "g_ptr: Null pointer!!!" )
		.get_item_number()
	}
}

#[no_mangle]
pub extern "C" fn generator_f64_get_generated_len( g_ptr: *const gen_struct_f64 ) -> size_t {
	unsafe{
		( g_ptr as *mut generator::Generator<f64> ).as_ref().expect( "g_ptr: Null pointer!!!" )
		.get_generated_len() as size_t
	}
}

#[no_mangle]
pub extern "C" fn generator_f64_get_generated_type( g_ptr: *const gen_struct_f64, idx: size_t ) -> gen_type {
	unsafe{
		( g_ptr as *mut generator::Generator<f64> ).as_ref().expect( "g_ptr: Null pointer!!!" )
		.get_generated_type( idx as usize )
	}
}

#[no_mangle]
pub extern "C" fn generator_f64_get_generated_rect( g_ptr: *const gen_struct_f64, idx: size_t ) -> *const gen_rect_f64 {
	unsafe{
		use geom::rect::Rect;
		let r = ( g_ptr as *mut generator::Generator<f64> ).as_ref().expect( "g_ptr: Null pointer!!!" )
		.get_generated_rect( idx as usize ) as *const Rect<f64>;
		std::mem::transmute::< *const Rect<f64>, *const gen_rect_f64 >( r )
	}
}

#[no_mangle]
pub extern "C" fn generator_f64_get_generated_center( g_ptr: *const gen_struct_f64, idx: size_t ) -> gen_point_f64 {
	unsafe{
		use geom::point::Point;
		let r = ( g_ptr as *mut generator::Generator<f64> ).as_ref().expect( "g_ptr: Null pointer!!!" )
		.get_generated_rect( idx as usize );
		let p = r.center();
		std::mem::transmute::< Point<f64>, gen_point_f64 >( p )
	}
}

#[no_mangle]
pub extern "C" fn generator_f64_get_generated_vector( g_ptr: *const gen_struct_f64, idx: size_t ) -> *const gen_vector_f64 {
	unsafe{
		use geom::vector::Vector;
		let v = ( g_ptr as *mut generator::Generator<f64> ).as_ref().expect( "g_ptr: Null pointer!!!" )
		.get_generated_vector( idx as usize ) as *const Vector<f64>;
		std::mem::transmute::< *const Vector<f64>, *const gen_vector_f64>( v )
	}
}
