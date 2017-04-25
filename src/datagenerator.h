/*
 * Copyright 2016 <copyright holder> <email>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

#ifndef DATAGENERATOR_H
#define DATAGENERATOR_H

namespace datagen{

#include "gendata.h"
	
class DataGenerator {

public:
	
	DataGenerator(){
		generator_f32 = generator_f32_new();
	}

	~DataGenerator(){
		generator_f32_delete( generator_f32 );
	}
	
	bool generate(){
		return generator_f32_generate( generator_f32 );
	}

	bool distribute(){
		return generator_f32_distribute( generator_f32 );
	}
	
	size_t len(){
		return generator_f32_get_generated_len( generator_f32 );
	}
	
	void set_size( float width, float height ){
		generator_f32_set_size( generator_f32, width, height );
	}
	
	void get_size( float* width, float* height  ){
		generator_f32_get_size( generator_f32, width, height );
	}
	
	void set_item_number( size_t num ){
		generator_f32_set_item_number( generator_f32, num ); 
	}

	size_t get_item_number(){
		return generator_f32_get_item_number( generator_f32 ); 
	}
	
	gen_rect_f32 const* get_rect( size_t idx ){
		return generator_f32_get_generated_rect( generator_f32, idx );
	}
	
	gen_vector_f32 const* get_vect( size_t idx ){
		return generator_f32_get_generated_vector( generator_f32, idx );
	}

	gen_type get_type( size_t idx ){
		return generator_f32_get_generated_type( generator_f32, idx );
	}

	void get_center( size_t idx, gen_point_f32* center  ){
		return generator_f32_get_generated_rect_center( generator_f32, idx, center );		
	}
	
private:
	gen_struct_f32* generator_f32;
};

}
#endif // DATAGENERATOR_H
