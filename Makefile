CFLAGS=-std=c99 -Wall -pedantic -g3
game:
	clang *.c `pkg-config --libs --cflags raylib` -o run
