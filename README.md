# IFS Fractal Visualizer
This is a toy program to render [Iterated Function System](https://en.wikipedia.org/wiki/Iterated_function_system) fractals with Rust and OpenGL. [Glium](https://github.com/glium/glium) is used for OpenGL and the GUI is provided by [imgui-rs](https://github.com/Gekkio/imgui-rs).

## Basic Idea
An IFS consists of several functions `f(x_0, y_0) -> (x, y)`, each with a given probability. To construct the fractal, start at the origin and do the following:

1. Draw a point at the current location
2. Select a function `f` at random from the IFS
3. Update the current location by applying `f` to it
4. Repeat

This program implements a linear transform with 6 parameters `a` through `f`: `x = a * x_0 + b * y_0 + e`, `y = c * x_0 + d * y_0 + f`.

## Building and running
Just use Cargo: `$ cargo run --release`

## GIF
The default IFS is a Barnsley's Fern. Here's a GIF of the visualizer:

![screenshot](https://thumbs.gfycat.com/IncredibleUnequaledAppaloosa-size_restricted.gif)
