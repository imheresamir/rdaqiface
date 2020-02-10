cmake_minimum_required(VERSION 3.0)
project(LibDaqIface C)

add_library(daqiface STATIC daqiface.c)

install(TARGETS daqiface DESTINATION .)
