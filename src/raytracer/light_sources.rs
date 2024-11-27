use crate::raytracer::{MatVec, Color, InputState};
use crate::raytracer::scene::LightSource;
use crate::raytracer::ray::Ray;

/// Sun is a light source that is infinitely far away and shines in a single direction.
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

/// Bulb is a light source that is located at a specific position and shines in all directions.
/// The intensity of the light decreases with the square of the distance.
pub struct Bulb {
    pub position: MatVec<3>,
    pub color: Color,
}

impl Bulb {
    pub fn new(position: MatVec<3>, context: &InputState) -> Bulb {
        Bulb {
            position,
            color: context.color.clone(),
        }
    }
}

impl LightSource for Bulb {
    fn light_color(&self) -> Color {
        self.color.clone()
    }
    
    fn intensity(&self, ray: &Ray) -> f32 {
        let distance: f32 = (self.position.clone() - ray.origin.clone()).magnitude();
        1.0f32 / f32::powi(distance, 2)
    }

    fn compute_direction(&self, origin: &MatVec<3>) -> MatVec<3> {
        (self.position.clone() - origin.clone()).normalize()
    }
}