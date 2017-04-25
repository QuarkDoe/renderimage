cmake_minimum_required(VERSION 3.2)
project(GenData)

find_program( CARGO cargo )

if( CARGO STREQUAL "CARGO-NOTFOUND" )
	message( FATAL_ERROR "`cargo` not found" )
else ()
	message( STATUS "`cargo` found" )
endif()

message( STATUS "CARGO [${CARGO}]" )
message( STATUS "CMAKE_BUILD_TYPE [${CMAKE_BUILD_TYPE}]" )

set( GENDATA_INCLUDE_DIR ${CMAKE_CURRENT_LIST_DIR}/target/binder/ PARENT_SCOPE )
if( CMAKE_BUILD_TYPE STREQUAL "Release" )
	set ( BUILD_TYPE "--release" )
	set( GENDATA_LIBRARY_DIR ${CMAKE_CURRENT_LIST_DIR}/target/release/ PARENT_SCOPE )
else()
	set( GENDATA_LIBRARY_DIR ${CMAKE_CURRENT_LIST_DIR}/target/debug/ PARENT_SCOPE )
endif()

add_custom_target(
	build_gendata
	COMMAND ${CARGO} build ${BUILD_TYPE}
	WORKING_DIRECTORY ${CMAKE_CURRENT_LIST_DIR}
)

add_custom_target(
	clean_gendata
	COMMAND ${CARGO} clean ${BUILD_TYPE}
	WORKING_DIRECTORY ${CMAKE_CURRENT_LIST_DIR}
)

add_custom_target(
	test_gendata
	COMMAND ${CARGO} test ${BUILD_TYPE} -j 1 -- --nocapture
	WORKING_DIRECTORY ${CMAKE_CURRENT_LIST_DIR}
)
