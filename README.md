# rust-raytrace
Playground for Rust; hoping to build out something graphical, perhaps a ray tracer

## cleanup warnings

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
1. Clean up warnings, better error handling
1. Materials + Meshes
1. Window data structures (ie, window dimensions, clock)
1. camera / perspective
1. render something with textures
1. box 2d or equiv
1. entity component system

## Links
* [The Rust Book](https://doc.rust-lang.org/book/)
* [storage buf example](https://www.geeks3d.com/20140704/tutorial-introduction-to-opengl-4-3-shader-storage-buffers-objects-ssbo-demo/)
* [objectified](https://www.tomdalling.com/blog/modern-opengl/05-model-assets-and-instances/)
* [storage buf history](https://github.com/lorenmh/rust-raytrace/blob/159ebbf2522af974c9828b71b8909e909e74037f/src/main.rs#L132-L139)
