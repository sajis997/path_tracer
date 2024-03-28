use crate::world::World;

pub trait Accelerator {
    fn accelerate(world: &World);
}