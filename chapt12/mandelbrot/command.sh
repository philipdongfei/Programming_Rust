#!/bin/bash
cargo build --release
time target/release/mandelbrot mandel.png 4000x3000 -1.20,0.35 -1.0,0.20

