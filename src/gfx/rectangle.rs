use nalgebra as na;

pub fn new(x: f32, y: f32, width: f32, height: f32, color: crate::gfx::Color) -> crate::gfx::object::Object {
    let mesh: std::vec::Vec<crate::gfx::Triangle> = vec![
        [
            na::Point3::new(-width/2.0,  height/2.0,  0.0), // top left corner
            na::Point3::new( width/2.0,  height/2.0,  0.0), // top right corner
            na::Point3::new( width/2.0, -height/2.0,  0.0), // bottom right corner
        ],
        [
            na::Point3::new(-width/2.0,  height/2.0,  0.0), // top left corner
            na::Point3::new(-width/2.0, -height/2.0,  0.0), // bottom left corner
            na::Point3::new( width/2.0, -height/2.0,  0.0), // bottom right corner
        ]
    ];

    let colorFn: fn(i32) -> [f32; 3] = |i| {
        if (i / 3) == 0 {
           return [0.2, 0.2, 0.7];
        }
        return [0.7, 0.3, 0.3];
    };

    crate::gfx::object::Object {
        pos: na::Vector3::new(  x,   y, 0.0),
        vel: na::Vector3::new(0.0, 0.0, 0.0),
        rot: na::Vector3::new(0.0, 0.0, 0.0),
        scale: 1.0,
        color: colorFn,
        mesh,
    }
}
