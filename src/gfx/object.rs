use nalgebra as na;

//fn position(&self) -> na::Vector3<f32>;
//fn rotation(&self) -> na::Vector3<f32>;
//fn scale(&self) -> f32;
//fn mesh(&self) -> Mesh;
//fn color(&self) -> Color;

pub struct Object {
    pub pos: na::Vector3<f32>,
    pub vel: na::Vector3<f32>,
    pub rot: na::Vector3<f32>,
    pub scale: f32,

    // TODO: move the mesh / color data to own modules
    pub mesh: crate::gfx::Mesh,
    pub color: crate::gfx::Color,
}

impl Object {
    pub fn to_string(&self) -> std::string::String {
       format!(
           "pos: {:?}\nvel: {:?}\nrot: {:?}\nscale: {:?}\ncolor: {:?}\nmesh: {:?}",
           self.pos,
           self.vel,
           self.rot,
           self.scale,
           self.color,
           self.mesh,
       )
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "pos: {:?}\nvel: {:?}\nrot: {:?}\nscale: {:?}\ncolor: {:?}\nmesh: {:?}",
            self.pos,
            self.vel,
            self.rot,
            self.scale,
            self.color,
            self.mesh,
        )
    }
}

impl crate::gfx::Render for Object {
    fn position(&self) -> na::Vector3<f32> { self.pos.clone() }
    fn rotation(&self) -> na::Vector3<f32> { self.rot.clone() }
    fn scale(&self) -> f32 { self.scale }
    fn mesh(&self) -> &crate::gfx::Mesh{ &self.mesh }
    fn color(&self) -> crate::gfx::Color{ self.color.clone() }
}
