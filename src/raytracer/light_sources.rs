use crate::raytracer::{MatVec, Color, InputState};
use crate::raytracer::scene::LightSource;
use crate::raytracer::ray::Ray;

pub struct Sun {
    pub direction: MatVec<3>,
    pub color: Color,
}


impl Sun {
    pub fn new(direction: MatVec<3>, context: &InputState) -> Sun {
        Sun {
            direction: direction.normalize(),
            color: context.color.clone(),
        }
    }
}


impl LightSource for Sun {
    fn light_color(&self) -> Color {
        self.color.clone()
    }
    fn intensity(&self, _ray: &Ray) -> f32 {
        1.0f32
    }

    fn compute_direction(&self, _origin: &MatVec<3>) -> MatVec<3> {
        self.direction.clone()
    }

}