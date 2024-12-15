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

    // TODO: Restructure these things to be propogated by the Objects themselves
    // IE I want the computations to be handled by each implementation of the SceneObject trait

    // add normal, and apply roughness to the normal
    fn propogate(&self, incident: &Ray, intersection: &Intersection) -> Ray;
    fn shininess(&self) -> Option<Vec<f32>> { None }
    fn transparency(&self) -> Option<Vec<f32>> { None }
    // fn roughness(&self) -> Option<f32> { None }
    fn ior(&self) -> f32 { 1.458 }
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

pub trait Material {
    fn compute_shine(&self, point: &Intersection) -> f32;
    fn compute_color(&self, point: &Intersection) -> Color;
    fn compute_intensity(&self, point: &Intersection) -> f32;
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

    pub fn find_minimum_intersection_with_point(&self, ray: &Ray, intersection: &IntersectionPayload) -> IntersectionPayload {
        
        if intersection.is_none() {
            return self.find_minimum_intersection(ray);
        }

        let intersection = intersection.clone().unwrap();

        let mut intersections: Vec<Intersection> = Vec::new();

        let mut local_ray = ray.clone();

        for (i, shape) in self.shapes.iter().enumerate() {

            // Bias the origin if the intesection belongs to this shape
            if intersection.shape_id.unwrap() == i {
                local_ray.origin = local_ray.origin.clone() + 0.055f32 * local_ray.direction.clone();
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
    /// Utilizes the `_recursive_raytrace` method to handle recursive raytracing.
    pub fn trace_ray(&self, ray: &Ray, bounce_limit: u32) -> RGBA {
        self._recursive_raytrace(ray,  &None, bounce_limit)
    }

    /// Recursive implementation of raytracing, with support for reflections and transparency.
    fn _recursive_raytrace(&self, ray: &Ray, optional_intersection: &IntersectionPayload, bounce_limit: u32) -> RGBA {
        // cast primary ray

        let primary_colision: IntersectionPayload = self.find_minimum_intersection_with_point(ray, &optional_intersection);

        if primary_colision.is_none() {

            return MatVec::new(vec![0.0, 0.0, 0.0, 0.0]);

        }

        let colision: Intersection = primary_colision.unwrap(); // consume the payload

        let shape_id: usize = colision.shape_id.unwrap();
        let mut color: Color = self.shapes[shape_id].color_at(&colision.point);

        let ilumination_sources: Vec<LightResidual> = self._find_light_sources(&colision);

        // In a shadow, return black
        if ilumination_sources.is_empty() {

            return MatVec::new(vec![0.0, 0.0, 0.0, 1.0]);

        }

        // Handle shininess and transparency
        let shininess = self.shapes[shape_id].shininess().unwrap_or(vec![0.0, 0.0, 0.0]);
        let transparency = self.shapes[shape_id].transparency().unwrap_or(vec![0.0, 0.0, 0.0]);

        let mut reflection_color: Color = MatVec::new(vec![0.0, 0.0, 0.0]);
        let mut refraction_color: Color = MatVec::new(vec![0.0, 0.0, 0.0]);

        if bounce_limit > 1 {
            // Handle reflections
            if shininess.iter().any(|&s| s > 0.0) {
                let reflection_ray = Ray::generate_reflection_ray(&colision.clone(), ray);
                reflection_color = utils::rgba_to_color(self._recursive_raytrace(&reflection_ray, &Some(colision.clone()), bounce_limit - 1));

                // // Combine reflection color with base color using per-channel shininess
                // for i in 0..3 {
                //     color.set(i, (1.0 - shininess[i]) * color[i] + shininess[i] * reflection_color[i]);
                // }
            }

            // Handle refractions
            // TODO: Make sure that tranceparency is suppoed to be double applies on the in and out
            if transparency.iter().any(|&t| t > 0.0) {
                let mut refraction_ray = Ray::generate_refraction_ray(&colision, ray, self.shapes[shape_id].ior());
                // utils::in_place_propogate(&mut refraction_ray);
                let pass_through_intersection = self.find_minimum_intersection_with_point(&refraction_ray, &Some(colision.clone()));
                refraction_color = utils::rgba_to_color(self._recursive_raytrace(&refraction_ray, &Some(colision.clone()), bounce_limit - 1));

                // Combine refraction color with base color using per-channel transparency
                // for i in 0..3 {
                //     color.set(i, (1.0 - shininess[i]) * ((1.0 - transparency[i]) * color[i] + transparency[i] * refraction_color[i]));
                // }
            }
        }

        // Apply Lambertian shading

        for i in 0..3 {
            let color_val: f32 = shininess[i] * reflection_color[i]
                                    + (1_f32 - shininess[i]) * transparency[i] * refraction_color[i]
                                    + (1_f32 - shininess[i]) * (1_f32 - transparency[i]) * color[i];
            color.set(i, color_val);
        }

        return utils::lambert(&color, &ilumination_sources);

    }

    /// Returns all light sources that illuminate an intersection
    fn _find_light_sources(&self, primary_intersection: &Intersection) -> Vec<LightResidual> {

        let mut light_sources: Vec<LightResidual> = Vec::new();

        for (_i, light_source) in self.light_sources.iter().enumerate() {

            let mut current_residual: LightResidual = LightResidual::new();
            let light_ray: Ray = Ray::generate_light_ray(&primary_intersection, light_source);

            let intersection = self.find_minimum_intersection_with_point(&light_ray, &Some(primary_intersection.clone()));

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