cmake_minimum_required(VERSION 3.0)

project(telnet_with_cpp_windows)

# Define the source files for the project
set(SOURCES
    StringBuilder.cpp
    telnet_with_cpp_windows.cpp
)

# Add the header files to the project
set(HEADERS
    StringBuilder.hpp
)

# Create an executable target for the project
add_executable(telnet_with_cpp_windows ${SOURCES} ${HEADERS})

##to build the project
##mkdir build
##cd build
##cmake ..
##make (only needed in linux maybe) on windows use cmake --build . instead