//
//
//
//

#ifndef _UTILS_CONTAINER_
#define _UTILS_CONTAINER_

#include <string>
#include <vector>
#include <list>
#include "exception.hpp"

namespace utils {

/**
 * @brief контейнер одного значения.
 **/
template <typename _Tp> class container {
public:
	container(): _defined( false ) {
	}

	container( _Tp v ): _value( v ), _defined( true ) {
	}

	container( const container<_Tp>& cv ): _value( cv.get() ), _defined( true ) {
	}

	bool defined() {
		return _defined;
	}

	void set( _Tp& v ) throw( access_exception ) {
		if( _defined ) throw access_exception( __PRETTY_FUNCTION__, "value already defined" );
		_value = v;
		_defined = true;
	}

	_Tp& get() throw( access_exception ) {
		if( !_defined ) throw access_exception( __PRETTY_FUNCTION__, "value undefined" );
		else return _value;
	}

	const _Tp& get() const throw( access_exception ) {
		if( !_defined ) throw access_exception( __PRETTY_FUNCTION__, "value undefined" );
		else return _value;
	}

	void reset() {
		_defined = false;
	}

	void reset( _Tp& v ) {
		_value = v;
		_defined = true;
	}

	_Tp& operator* () throw( access_exception ) {
		if( !_defined ) throw access_exception( __PRETTY_FUNCTION__, "value undefined" );
		else return _value;
	}

	container<_Tp >& operator= ( const _Tp& v ) throw( access_exception ) {
		if( _defined ) throw access_exception( __PRETTY_FUNCTION__, "value already defined" );
		_value = v;
		_defined = true;
		return *this;
	}

	container<_Tp >& operator= ( const container<_Tp >& v ) throw( access_exception ) {
		if( _defined ) throw access_exception( __PRETTY_FUNCTION__, "value already defined" );
		if( this == &v ) return *this;
		_value = v.get();
		_defined = true;
		return *this;
	}

	operator _Tp() throw( access_exception ) {
		if( !_defined ) throw access_exception( __PRETTY_FUNCTION__, "value undefined" );
		else return _value;
	}

protected:
	bool _defined;
	_Tp _value;
};

typedef container<int>    int_c;
typedef container<double> double_c;
typedef container<float>  float_c;
typedef container<bool>   bool_c;
typedef container<std::string> string_c;

/**
 * @brief контейнер одного значения.
 **/
template <typename _Tp> class weak_container : public container<_Tp> {
public:
	weak_container(): container<_Tp>() {
	}

	weak_container( _Tp v ): container<_Tp>( v ) {
	}

	weak_container( const container<_Tp>& cv ): container<_Tp>( cv ) {
	}

	void set( _Tp& v ) throw( access_exception ) {
		container<_Tp>::_value = v;
		container<_Tp>::_defined = true;
	}

	weak_container<_Tp >& operator= ( const _Tp& v ) throw( access_exception ) {
		container<_Tp>::_value = v;
		container<_Tp>::_defined = true;
		return *this;
	}

	weak_container<_Tp >& operator= ( const container<_Tp >& v ) throw( access_exception ) {
		if( this == &v ) return *this;
		container<_Tp>::_value = v.get();
		container<_Tp>::_defined = true;
		return *this;
	}

};

typedef weak_container<int>    int_wc;
typedef weak_container<double> double_wc;
typedef weak_container<float>  float_wc;
typedef weak_container<bool>   bool_wc;
typedef weak_container<std::string> string_wc;

template < class T >
class auto_ptr_vector : public std::vector<T*> {
//private:
public:
	typedef std::vector<T*> p;
	typedef typename std::vector<T*>::iterator pt;

	auto_ptr_vector(): p() {
	}

	auto_ptr_vector( int v ): p( v ) {
	}

	auto_ptr_vector( p v ): p( v ) {
	}

	~auto_ptr_vector() {
		for( int i = 0; i < p::size(); i++ ) {
			T* ptr = p::at( i );
			if( ptr ) delete ptr;
		}
		p::clear();
		//((p*)this)->~p();
	}
};

template < class T >
class auto_ptr_list : public std::list<T*> {

public:
	typedef std::list<T*> p;
	typedef typename std::list<T*>::iterator pt;

	auto_ptr_list(): p() {
	}

	auto_ptr_list( int v ): p( v ) {
	}

	auto_ptr_list( p v ): p( v ) {
	}

	~auto_ptr_list() {
		clear();
	}

	void clear() {
		;
		for( pt it = p::begin(); it != p::end(); it++ ) {
			T* ptr = *it;
			if( ptr ) delete ptr;
		}
		p::clear();
	}
};

template <class T>
/**
 * @brief conteiner for pointers created with new, at destruction called delete for stored pointer
 **/
class auto_ptr {
public:
	auto_ptr(): data( NULL ) {}

	auto_ptr( T* a ): data( a ) {}
	
	inline T* get() {
		return data;
	}
	
	inline bool empty(){
		return data == NULL;
	}

	inline T* release() {
		T* ret = data;
		data = NULL;
		return ret;
	}

	inline T& operator * () {
		return *data;
	}
	
	inline const T& operator * () const {
		return *data;
	}
	
	inline auto_ptr<T>& operator = ( T* a ) {
		if( data ) delete data;
		data = a;
		return *this;
	}
	
	inline T* operator-> () {
		return data;
	}
	
	inline operator T* () {
		return data;
	}
	
// 	inline operator const T* () {
// 		return data;
// 	}
// 	
// 	inline operator bool() const {
// 		return data != NULL;
// 	}

	~auto_ptr() {
		if( data ) delete data;
	};
private:
	T* data;
};

template <class T>
/**
 * @brief conteiner for arrays created with new[], at destruction called delete[] for stored pointer
 **/
class auto_arr_ptr {
public:
	auto_arr_ptr(): data( 0 ) {}
	auto_arr_ptr( T* a ): data( a ) {}
	
	inline T* get() {
		return data;
	}

	inline T* release() {
		T* ret = data;
		data = NULL;
		return ret;
	}
	
	inline void reset( T* v ) {
		if( data != NULL ) delete[] data;
		data = v;
	}
	
	inline bool empty() const{
		return ( this->data == NULL );
	}
	
	inline T& operator * () {
		return *data;
	}
	
	inline const T& operator * () const {
		return *data;
	}
	
	inline T& operator []( int i ) {
		return data[i];
	}
	
	inline const T& operator []( int i ) const {
		return data[i];
	}
	
	inline auto_arr_ptr<T>& operator = ( T* a ) {
		if( data != NULL ) delete[] data;
		data = a;
		return *this;
	}

	inline T* operator-> () {
		return data;
	}

	inline operator T* () {
		return data;
	}
	
// 	inline operator const T* () {
// 		return data;
// 	}

// 	inline operator bool() const {
// 		return data != NULL;
// 	}

	~auto_arr_ptr() {
		if( data )
			delete[] data;
	};
private:
	T* data;
};

}

#endif
