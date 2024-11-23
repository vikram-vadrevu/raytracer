use crate::raytracer::{Intersection, IntersectionPayload, MatVec, RGBA};
use crate::raytracer::ray::Ray;
use crate::raytracer::utils;

use super::{Light, LightResidual};

pub trait SceneObject {
    fn intersect(&self, ray: &Ray) -> IntersectionPayload;
    fn normal(&self, point: &MatVec) -> MatVec;
    fn color_at(&self, point: &MatVec) -> RGBA;
    fn apply_dir_transform(&self, dir: &MatVec) -> MatVec;
    fn apply_light_transform(&self, light: &MatVec) -> MatVec;
}

pub trait LightSource {
    // Compute the -1 * direction vector from the light source to the point
    fn compute_inverse_direction(&self, origin: &MatVec) -> MatVec;
    fn light_color(&self) -> RGBA;
    fn intensity(&self, ray: &Ray) -> f32;
    // fn position(&self) -> MatVec;
    // fn color(&self) -> RGBA;
}

pub struct Scene {
    pub shapes: Vec<Box<dyn SceneObject>>,
    // pub materials: Vec<Material>,
    pub light_sources: Vec<Box<dyn LightSource>>
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            shapes: Vec::new(),
            // materials: Vec::new(),
            light_sources: Vec::new()
        }
    }

    pub fn add_shape(&mut self, shape: Box<dyn SceneObject>) {
        self.shapes.push(shape);
    }

    pub fn find_any_intersection(&self, ray: &Ray) -> IntersectionPayload {
        for shape in &self.shapes {
            let intersection: IntersectionPayload = shape.intersect(&ray);
            if intersection.is_some() {
                return intersection;
            }
        }
        None
    }

    pub fn find_minimum_intersection(&self, ray: &Ray) -> IntersectionPayload {
        let mut intersections: Vec<Intersection> = Vec::new();
        // for i, shape in &self.shapes {
        for (i, shape) in self.shapes.iter().enumerate() {
            let mut intersection: IntersectionPayload = shape.intersect(&ray);
            if intersection.is_some() {
                intersection.as_mut().unwrap().shape_id = Some(i);
                intersections.push(intersection.unwrap());
            }
        }
        if intersections.is_empty() {
            return None;
        }

        let mut minimum_intersection: IntersectionPayload = None;

        for i in intersections {
            if minimum_intersection.is_none() || i.distance < minimum_intersection.as_ref().unwrap().distance {
                minimum_intersection = Some(i);
            }
        }
        println!("Minimum intersection: {:?}", minimum_intersection);
        minimum_intersection
    }

    // Traces a ray through the scene and returns the color at the intersection
    // of the primary ray and its collision in the scene
    pub fn trace_through_scene(&self, ray: &Ray, bounce_limit: u32) -> RGBA {
        // cast primary ray
        let primary_colision: IntersectionPayload = self.find_minimum_intersection(ray);

        if primary_colision.is_none() {
            return MatVec::new(vec![0.0, 0.0, 0.0, 0.0]);
        }

        let shape_id: usize = primary_colision.as_ref().unwrap().shape_id.unwrap();
        let color: RGBA = self.shapes[shape_id].color_at(&primary_colision.as_ref().unwrap().point);

        if bounce_limit > 1 {
            // cast secondary rays
            // for each light source, calculate a ray to the light source
            // and check the scene for intersections, if intersected, apply shadow and return
            todo!("Secondary rays not yet implemented");
        }
        
        let ilumination_sources: Vec<LightResidual> = self._find_light_sources(primary_colision.as_ref().unwrap());
        // let total_light_contribution: MatVec = self._attenuate_light(ilumination_sources);
        // In a shadow
        if ilumination_sources.is_empty() {
            return MatVec::new(vec![0.0, 0.0, 0.0, 1.0]);
        }

        return utils::lambert(color, ilumination_sources);
    }

    fn _recursive_trace_through_scene(intersection: Intersection, bounce_limit: u32) -> Intersection {
        // cast secondary rays
        // for each light source, calculate a ray to the light source
        // and check the scene for intersections, if intersected, apply shadow and return
        todo!("Secondary rays not yet implemented");
    }

    // Returns all light sources that illuminate an intersection
    fn _find_light_sources(&self, Intersection: &Intersection) -> Vec<LightResidual> {
        let mut light_sources: Vec<LightResidual> = Vec::new();
        for (i, light_source) in self.light_sources.iter().enumerate() {
            let mut current_residual = LightResidual::new();
            let light_ray = Ray::generate_light_ray(&Intersection, light_source);
            let intersection = self.find_any_intersection(&light_ray);
            if intersection.is_none() {
                current_residual.color = light_source.light_color();
                current_residual.intensity = light_source.intensity(&light_ray);
                current_residual.direction = light_ray.inverse();
            }
        }
        light_sources
    }

    // fn _recursive_resolve_residual() -> ColisionRedisual {
        
    // }

}