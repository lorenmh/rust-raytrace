use nalgebra as na;

pub struct Axes {
    pub phys: crate::physics::Physics,
    pub gfx: crate::gfx::render::Renderer,
}

pub fn new() -> Axes {
    let x = crate::shapes::rectangle::new(
        0.0,
        0.0,
        0.0,
        1000.0,
        0.2,
        |i| { [1.0, 0.0, 0.0] },
    );

    let y = crate::shapes::rectangle::new(
        0.0,
        0.0,
        0.0,
        0.2,
        1000.0,
        |i| { [0.0, 1.0, 0.0] },
    );

    let mut z = crate::shapes::rectangle::new(
        0.0,
        0.0,
        0.0,
        0.2,
        1000.0,
        |i| { [0.0, 0.0, 1.0] },
    );
    z.phys.rot = na::Vector3::x() * -std::f32::consts::FRAC_PI_2;

    let mut mesh: std::vec::Vec<crate::gfx::Triangle> = x.vertices();
    mesh.extend(y.vertices());
    mesh.extend(z.vertices());

    Axes{
        phys: crate::physics::new(0.0, 0.0, 0.0),
        gfx: crate::gfx::render::new(
            1.0,
            mesh,
            |i| {
                if (i / 6) == 0 {
                    [1.0, 0.0, 0.0]
                } else if (i / 12) == 0 {
                    [0.0, 1.0, 0.0]
                } else {
                    [0.0, 0.0, 1.0]
                }
            },
        ),
    }
}

impl Axes {
    pub fn render(&self, params: &crate::gfx::render::Params) {
        self.gfx.render(&self.phys, params);
    }
}

