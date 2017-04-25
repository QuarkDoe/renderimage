cmake_minimum_required( VERSION 3.2 )
project( RenderImage )


# Find includes in corresponding build directories
set( CMAKE_INCLUDE_CURRENT_DIR ON )
# Instruct CMake to run moc automatically when needed.
set( CMAKE_AUTOMOC ON )
set( CMAKE_AUTORCC ON )

add_subdirectory( gendata )

include_directories( ${GENDATA_INCLUDE_DIR} )
link_directories( ${GENDATA_LIBRARY_DIR} )

# Find the QtWidgets library
find_package( Qt5Widgets REQUIRED )

set(
	renderimage_SRC
	src/main.cpp
	src/renderimage.cpp
	src/workerthread.cpp
)

set(
	renderimage_RES
	res/icons.qrc
)

# Create code from a list of Qt designer ui files.
#set(CMAKE_AUTOUIC ON) # use this if you have CMake 3.x instead of the following
qt5_wrap_ui(
	renderimage_SRC
	src/renderimage.ui
)

# Tell CMake to create the helloworld executable
add_executable( renderimage ${renderimage_SRC} ${renderimage_RES} )

add_dependencies( renderimage build_gendata )

# Use the Widgets module from Qt 5.
target_link_libraries( renderimage gendata Qt5::Widgets )

# Install the executable
install( TARGETS renderimage DESTINATION bin )
