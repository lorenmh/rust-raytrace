# rust-raytrace
Playground for Rust; hoping to build out something graphical, perhaps a ray tracer

## How
* SDL2 via the `sdl2` crate will provide window / OS independent media layer.
  - provides key input, windows, openGL context, etc
* use 
  
## What
### raytracer
Only requires `sdl2`. Would be a fun weekend project.
 
Technically, could just write directly to some buffer and then output as an image. However, if we want this to be "realtime" (ie, the user sees the scene generated before their eyes), would want this to be interactive with something like SDL.
 
### cross platform game
Requires `sdl2`, will want to use an OpenGL ES 2 context (most likely).
Likely something graphical which doesnt require too much native input (ie, no swipes, nothing crazy).

Ideas so far:
* something like "pikman", ie, some form of automata that the user interacts with, more "guiding" than "controlling" the automata
  - ex: https://www.youtube.com/watch?v=bqtqltqcQhw : these "boids", only have 3 rules (avoid colliding with boids, attempt to steer in a similar direction of other boids, attempt to move towards center mass of boids). A set of simple rules which dont require complex textures / crazy art assets.
* some kind of "train track utilitarian" game, where there's a runaway train and the user quickly change tracks before the train crashes or collides with pedestrians; a "frogger" if you will, except we're the train operator, not the frogs.

## Notes

## To Do
1. Read through the LearnOpenGL book
1. Better error handling (I assume GLSL compile error breaks code, just get a random SIGSEV error; make sure to handle errors)
1. Move all unsafe code to module
1. Figure out how matrices are generated
  - shader can generate transform matrices, however, does it generate a new matrix for each input vertex? if so, create using `nalgebra`
1. Simple "Hello Shader" program, which displays a single triangle, with vertex shader (x -> x) and a fragment shader that outputs red pixels within triangle
1. Simple "Hello Polyhedron" program, which displays a polyhedren with perspective projection
1. Get a better idea of the overall structure here

## Links
* ~[Learn GFX HAL](https://rust-tutorials.github.io/learn-gfx-hal/01_introduction.html)~
* [The Rust Book](https://doc.rust-lang.org/book/)
* [gl-rs example](https://gist.github.com/simias/c140d1479ada4d6218c0)
