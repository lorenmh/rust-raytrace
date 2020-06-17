use nalgebra as na;
use gl;
use std::ffi::CString;

pub mod camera;
pub mod shader;
pub mod render;


pub type Triangle = [na::Point3<f32>; 3];
pub type Mesh = std::vec::Vec<Triangle>;
pub type Color = [f32; 3];
pub type ColorFn = fn(i32) -> Color;

