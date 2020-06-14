use nalgebra as na;
use gl;
use std::ffi::CString;

pub mod camera;
pub mod object;
pub mod shader;
pub mod render;

pub mod rectangle;

pub type Triangle = [na::Point3<f32>; 3];
pub type Mesh = std::vec::Vec<Triangle>;
pub type Color = [f32; 3];

