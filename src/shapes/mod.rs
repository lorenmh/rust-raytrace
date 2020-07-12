use nphysics3d::object::{BodySet, ColliderSet, BodyHandle};
use nphysics3d::force_generator::{ForceGeneratorSet,};
use nphysics3d::joint::{JointConstraintSet,};

pub mod rectangle;
pub mod cube;
pub mod axes;

pub struct Environment {
    bodies: BodySet<f32>,
    colliders: ColliderSet<f32, BodyHandle>,
    forces: ForceGeneratorSet<f32, BodyHandle>,
    joints: JointConstraintSet<f32, BodyHandle>,
}
