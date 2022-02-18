#!/bin/bash
test -z $RUSTC && RUSTC=rustc
test -z $GCC && GCC=gcc
$RUSTC --crate-type dylib pepa.rs && \
$GCC ./testpepa.c ./libpepa.so && \
./a.out
