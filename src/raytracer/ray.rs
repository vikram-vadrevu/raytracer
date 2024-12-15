use crate::raytracer::{CameraState, Intersection, MatVec, ProjectionType};
use crate::raytracer::scene::LightSource;
use rand::Rng;

/// Represents a ray in 3D space.
/// The actual creating of rays is done through the
/// static methods implemented on this struct.
#[derive(Debug, Clone)]
pub struct Ray {

    pub origin: MatVec<3>,
    pub direction: MatVec<3>,

}

impl Ray {
    
    pub fn new(origin: MatVec<3>, direction: MatVec<3>) -> Ray {
        Ray {
            origin,
            direction,
        }
    }

    /// Generates a ray that passes through the pixel at the given coordinates.
    /// Origin of the ray is the camera's eye, and the direction is computed based on the projection type.
    /// Currently supportes, Flat and Fisheye projections.
    /// NOTE: Fisheye does not work as of now.
    #[allow(unreachable_patterns)]
    pub fn generate_primary_ray(through_pixel: MatVec<2>, context: &CameraState) -> Ray {
        
        match context.projection {
            
            ProjectionType::FLAT => {
                
                let s_x: f32 = ((2.0 * through_pixel[0]) - (context.width as f32)) / (u32::max(context.width, context.height) as f32);
                let s_y: f32 = ((context.height as f32) - (2.0 * through_pixel[1])) / (u32::max(context.width, context.height) as f32);

                let eye: MatVec<3> = context.eye.clone();
                let forward: MatVec<3> = context.forward.clone();

                let arbitrary_up = context.up.clone().normalize();

                // Compute the right vector
                let right: MatVec<3> = forward.cross(&arbitrary_up).normalize();

                // Compute the proper up vector to ensure orthonormality
                let up: MatVec<3> = right.cross(&forward).normalize();

                // Construct the ray direction
                let mut ray = Ray::new(eye.clone(), (forward + (s_x * right) + (s_y * up)).normalize());

                // Apply depth of field if enabled
                if let Some(dof_params) = &context.dof {
                    let focus = dof_params[0];
                    let lens_radius = dof_params[1];

                    // Randomly perturb the ray's origin and direction
                    let mut rng = rand::thread_rng();
                    let rand_x: f32 = rng.gen_range(-1.0..1.0);
                    let rand_y: f32 = rng.gen_range(-1.0..1.0);

                    let lens_offset = lens_radius * (rand_x * right + rand_y * up);
                    let new_origin = eye + lens_offset;

                    let focal_point = ray.origin + focus * ray.direction;
                    let new_direction = (focal_point - new_origin).normalize();

                    ray = Ray::new(new_origin, new_direction);
                }

                ray

            },

            ProjectionType::FISHEYE => {

                let s_x: f32 = ((2.0 * through_pixel[0]) - (context.width as f32)) / (u32::max(context.width, context.height) as f32);
                let s_y: f32 = ((context.height as f32) - (2.0 * through_pixel[1])) / (u32::max(context.width, context.height) as f32);

                if s_x.powi(2) + s_y.powi(2) > 1.0 {
                    return Ray::new(MatVec::new(vec![0.0, 0.0, 0.0]), MatVec::new(vec![0.0, 0.0, 0.0])); // No ray
                }

                let eye: MatVec<3> = context.eye.clone();
                let forward: MatVec<3> = context.forward.clone();
                let arbitrary_up = context.up.clone().normalize();
                let right: MatVec<3> = forward.cross(&arbitrary_up).normalize();
                let up: MatVec<3> = right.cross(&forward).normalize();
                let direction = (f32::sqrt(1.0 - s_x.powi(2) - s_y.powi(2)) * forward + s_x * right + s_y * up).normalize();

                Ray::new(eye, direction)

            }

            ProjectionType::PANORAMIC => {
                let theta: f32 = (through_pixel[0] / context.width as f32) * 2.0 * std::f32::consts::PI - std::f32::consts::PI;
                let phi: f32 = ((context.height as f32 - through_pixel[1]) / context.height as f32) * std::f32::consts::PI - (std::f32::consts::PI / 2.0);
            
                let eye: MatVec<3> = context.eye.clone();
                let forward: MatVec<3> = context.forward.clone();
                let arbitrary_up = context.up.clone().normalize();
                let right: MatVec<3> = forward.cross(&arbitrary_up).normalize();
                let up: MatVec<3> = right.cross(&forward).normalize();
                let direction = (theta.cos() * phi.cos() * forward + theta.sin() * phi.cos() * right + phi.sin() * up).normalize();
            
                Ray::new(eye, direction)
            }
            

            _ => todo!("Projection type {:?} is not yet supported", context.projection),
        }
    }


    /// Generates a ray that starts at the intersection point and points towards the light source.
    pub fn generate_light_ray(intersection: &Intersection, light_source: &Box<dyn LightSource>) -> Ray {
        let origin = intersection.point.clone();
        let dir = light_source.compute_direction(&origin);

        Ray::new(origin, dir)
    }

    // /// Phong reflection model
    // pub fn generate_reflection_ray(intersection: &Intersection, incoming_ray: &Ray) -> Ray {
    //     let origin = intersection.point.clone();
    //     let normal = intersection.normal.clone();
    //     let direction = incoming_ray.direction.clone();
    //     let reflection = direction  - 2.0 * direction.dot(normal) * normal;
    //
    //     Ray::new(origin, reflection.normalize())
    // }

    /// Phong reflection model
    pub fn generate_reflection_ray(intersection: &Intersection, incoming_ray: &Ray) -> Ray {
        
        // Slightly offset the origin to prevent self-intersection
        let origin = intersection.point.clone();

        // Ensure the normal and incoming direction are normalized
        let normal = intersection.normal.normalize();
        let direction = incoming_ray.direction.normalize();

        // Calculate reflection direction
        let dot = direction.dot(normal).clamp(-1.0, 1.0); // Clamp for numerical stability
        let reflection = (direction - 2.0 * dot * normal).normalize();

        Ray::new(origin, reflection)

    }


    pub fn generate_refraction_ray(intersection: &Intersection, incoming_ray: &Ray, ior: f32) -> Ray {
        let mut normal = intersection.normal.clone();
        let mut eta = 1.0 / ior; // Assume ray is entering the material

        // Check if the ray is exiting the material
        if incoming_ray.direction.dot(normal) > 0.0 {
            normal = -1.0f32 * normal;
            eta = ior;
        }

        let cos_i = -normal.dot(incoming_ray.direction.clone()).clamp(-1.0, 1.0);
        let sin_t2 = eta * eta * (1.0 - cos_i * cos_i);

        // Total internal reflection
        if sin_t2 > 1.0 {
            return Ray::generate_reflection_ray(intersection, incoming_ray);
        }

        let cos_t = (1.0 - sin_t2).sqrt();
        let refraction = eta * incoming_ray.direction + (eta * cos_i - cos_t) * normal;
        let origin = intersection.point.clone();
        Ray::new(origin, refraction.normalize())
    }
}