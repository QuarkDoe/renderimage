/*-------------------------------------------------------------------------
    c_except.hpp
-------------------------------------------------------------------------*/
#ifndef _UTILS_EXCEPT_
#define _UTILS_EXCEPT_

#include <string>
#include <exception>

#define SMLEX_GENERAL  1
#define SMLEX_FILEIO   10
#define SMLEX_FOPEN    (SMLEX_FILEIO + 1)
#define SMLEX_FREAD    (SMLEX_FILEIO + 2)
#define SMLEX_FWRITE   (SMLEX_FILEIO + 3)
#define SMLEX_ACCESS   20
#define SMLEX_REQUEST  30
#define SMLEX_PARAMS   40
#define SMLEX_PARSE    50

namespace utils {

class exception : public std::exception {
public:
	exception( const std::string& msg ): Id( SMLEX_GENERAL ), Text( msg ), Source( __PRETTY_FUNCTION__ ) {}
	exception( const std::string& src, const std::string& msg ): Id( SMLEX_GENERAL ), Text( msg ), Source( src ) {}
	exception( int n, const std::string& msg ): Id( n ), Text( msg ), Source( __PRETTY_FUNCTION__ ) {}
	exception( int n, const std::string& src, const std::string& msg ): Id( n ), Text( msg ), Source( src ) {}
	exception( const exception& e )	: Id( e.Id ), Text( e.Text ), Source( e.Source ) {}
	~exception() throw() {
		//Text.std::string::~string();
	}

	exception& operator = ( const exception& e ) {
		Text = e.Text;
		Id = e.Id;
		Source = e.Source;
		return *this;
	}

	int id() const {
		return Id;
	}

	const std::string& text() const {
		return Text;
	}

	const std::string& source() const {
		return Source;
	}

	const char* what() const throw() {
		return Text.c_str();
	}

	const char* ctext() const throw() {
		return Text.c_str();
	}

private:
	int Id;
	std::string Source;
	std::string Text;
};

class access_exception : public exception {
public:
	access_exception( const std::string& msg ): exception( SMLEX_ACCESS, __PRETTY_FUNCTION__, msg ) {}
	access_exception( const std::string& src, const std::string& msg ): exception( SMLEX_ACCESS, src, msg ) {}
	access_exception( int n, const std::string& msg ): exception( n, msg ) {}
	access_exception( int n, const std::string& src, const std::string& msg ): exception( n, src, msg ) {}
};

/**
 * 
 */
class io_exception : public exception {
public:
	io_exception( const std::string& msg ): exception( SMLEX_FILEIO, __PRETTY_FUNCTION__, msg ) {}
	io_exception( const std::string& src, const std::string& msg ): exception( SMLEX_FILEIO, src, msg ) {}
	io_exception( int n, const std::string& msg ): exception( n, msg ) {}
	io_exception( int n, const std::string& src, const std::string& msg ): exception( n, src, msg ) {}
};

/**
 * 
 */
class param_exception : public exception {
public:
	param_exception( const std::string& msg ): exception( SMLEX_PARAMS, __PRETTY_FUNCTION__, msg ) {}
	param_exception( const std::string& src, const std::string& msg ): exception( SMLEX_PARAMS, src, msg ) {}
	param_exception( int n, const std::string& msg ): exception( n, msg ) {}
	param_exception( int n, const std::string& src, const std::string& msg ): exception( n, src, msg ) {}
};

/**
 * 
 */
class request_exception : public exception {
public:
	request_exception( const std::string& msg ): exception( SMLEX_REQUEST, __PRETTY_FUNCTION__, msg ) {}
	request_exception( const std::string& src, const std::string& msg ): exception( SMLEX_REQUEST, src, msg ) {}
	request_exception( int n, const std::string& msg ): exception( n, msg ) {}
	request_exception( int n, const std::string& src, const std::string& msg ): exception( n, src, msg ) {}
};

/**
 * 
 */
class parse_exception : public exception {
public:
	parse_exception( const std::string& msg ): exception( SMLEX_PARSE, __PRETTY_FUNCTION__, msg ) {}
	parse_exception( const std::string& src, const std::string& msg ): exception( SMLEX_PARSE, src, msg ) {}
	parse_exception( int n, const std::string& msg ): exception( n, msg ) {}
	parse_exception( int n, const std::string& src, const std::string& msg ): exception( n, src, msg ) {}
};

}

#endif
