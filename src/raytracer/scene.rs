use crate::raytracer::{Intersection, IntersectionPayload, MatVec, RGBA, Color, LightResidual};
use crate::raytracer::ray::Ray;
use crate::raytracer::utils;

/// Trait that defines the required behavior of any object in a scene.
/// Notable methods are `intersect` and `color_at`, which are used to
/// calculate a possible intersection with a given ray and the color (including textures)
/// at any given point on the surface of the object.
pub trait SceneObject {
    fn intersect(&self, ray: &Ray) -> IntersectionPayload;
    fn color_at(&self, point: &MatVec<3>) -> Color;
    // fn apply_dir_transform(&self, dir: &MatVec<3>) -> MatVec<3>;
    // fn apply_light_transform(&self, light: &MatVec<3>) -> MatVec<3>;
}

/// Trait that defines the required behavior of any light source in a scene.
/// Notable methods are `compute_direction`, `light_color` and `intensity`, which are used to
/// calculate the direction of the light source, the color of the light and the intensity of the light.
pub trait LightSource {
    fn compute_direction(&self, origin: &MatVec<3>) -> MatVec<3>;
    fn light_color(&self) -> Color;
    fn intensity(&self, ray: &Ray) -> f32;
    // fn position(&self) -> MatVec;
    // fn color(&self) -> RGBA;
}

/// The `Scene` struct holds all the objects and light sources in the scene.
/// It additionally provides methods to trace rays through the scene.
/// The actual tracing of any given `Ray` is facilitated through the scene object,
/// although many of the calculations are offloaded to the objects themselves or other
/// utility functions that are provided my other modules.
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

    pub fn add_light_source(&mut self, light_source: Box<dyn LightSource>) {

        self.light_sources.push(light_source);

    }

    // pub fn find_any_intersection(&self, ray: &Ray) -> IntersectionPayload {
    //     for shape in &self.shapes {
    //         let intersection: IntersectionPayload = shape.intersect(&ray);
    //         if intersection.is_some() {
    //             return intersection;
    //         }
    //     }
    //     None
    // }

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

        // println!("Minimum intersection: {:?}", minimum_intersection);
        minimum_intersection

    }

    pub fn find_minimum_intersection_with_point(&self, ray: &Ray, intersection: &Intersection) -> IntersectionPayload {
        
        let mut intersections: Vec<Intersection> = Vec::new();

        let mut local_ray = ray.clone();

        for (i, shape) in self.shapes.iter().enumerate() {

            // Bias the origin if the intesection belongs to this shape
            if intersection.shape_id.unwrap() == i {
                local_ray.origin = local_ray.origin.clone() + 0.038f32 * local_ray.direction.clone();
            }

            let mut intersection: IntersectionPayload = shape.intersect(&local_ray);

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
        // println!("Minimum intersection: {:?}", minimum_intersection);
        minimum_intersection
    }

    /// Traces a ray through the scene and returns the color at the intersection
    /// of the primary ray and its collision in the scene.
    /// In the future, this method will be used to trace secondary rays as well.
    pub fn trace_through_scene(&self, ray: &Ray, bounce_limit: u32) -> RGBA {
        // cast primary ray
        let primary_colision: IntersectionPayload = self.find_minimum_intersection(ray);

        if primary_colision.is_none() {
            return MatVec::new(vec![0.0, 0.0, 0.0, 0.0]);
        }

        let shape_id: usize = primary_colision.as_ref().unwrap().shape_id.unwrap();
        let color: Color = self.shapes[shape_id].color_at(&primary_colision.as_ref().unwrap().point);

        if bounce_limit > 1 {
            // cast secondary rays
            // for each light source, calculate a ray to the light source
            // and check the scene for intersections, if intersected, apply shadow and return
            todo!("Secondary rays not yet implemented");
        }
        
        let ilumination_sources: Vec<LightResidual> = self._find_light_sources(primary_colision.as_ref().unwrap());

        // In a shadow
        if ilumination_sources.is_empty() {
            return MatVec::new(vec![0.0, 0.0, 0.0, 1.0]);
        }

        return utils::lambert(&color, &ilumination_sources);
    }


    /// Returns all light sources that illuminate an intersection
    fn _find_light_sources(&self, primary_intersection: &Intersection) -> Vec<LightResidual> {

        let mut light_sources: Vec<LightResidual> = Vec::new();

        for (_i, light_source) in self.light_sources.iter().enumerate() {

            let mut current_residual: LightResidual = LightResidual::new();
            let light_ray: Ray = Ray::generate_light_ray(&primary_intersection, light_source);

            let intersection = self.find_minimum_intersection_with_point(&light_ray, primary_intersection);

            if intersection.is_none() {

                current_residual.color = light_source.light_color();
                current_residual.intensity = light_source.intensity(&light_ray);
                current_residual.direction = light_ray.direction.clone();
                current_residual.normal = primary_intersection.normal.clone();
                light_sources.push(current_residual);

            }

        }

        light_sources

    }

}