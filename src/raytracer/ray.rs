use crate::raytracer::{CameraState, Intersection, MatVec, ProjectionType};
use crate::raytracer::scene::LightSource;

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
                let forward: MatVec<3> = context.forward.clone().normalize();

                // Choose an arbitrary up vector not parallel to forward
                let arbitrary_up: MatVec<3> = if forward[0].abs() < forward[1].abs() && forward[0].abs() < forward[2].abs() {
                    MatVec::new(vec![1.0, 0.0, 0.0])
                } else if forward[1].abs() < forward[2].abs() {
                    MatVec::new(vec![0.0, 1.0, 0.0])
                } else {
                    MatVec::new(vec![0.0, 0.0, 1.0])
                };

                // Compute the right vector
                let right: MatVec<3> = forward.cross(&arbitrary_up).normalize();

                // Compute the proper up vector to ensure orthonormality
                let up: MatVec<3> = right.cross(&forward).normalize();

                // Construct the ray direction
                Ray::new(eye, (forward + (s_x * right) + (s_y * up)).normalize())

            },

            ProjectionType::FISHEYE => {

                let s_x: f32 = ((2.0*through_pixel[0]) - (context.width as f32)) / (u32::max(context.width, context.height) as f32);
                let s_y: f32 = ((context.height as f32) - (2.0*through_pixel[1])) / (u32::max(context.width, context.height) as f32);

                let eye: MatVec<3> = context.eye.clone();
                let mut forward: MatVec<3> = context.forward.clone();
                let up: MatVec<3> = context.up.normalize();
                let right: MatVec<3> = forward.cross(&up).normalize();

                forward = f32::sqrt(1.0f32 - (s_x.powi(2) - s_y.powi(2))) * forward;

                Ray::new(eye, (forward + ((s_x * right) + (s_y * up))).normalize())

            }

            ProjectionType::PANORAMIC => {
                let s_x: f32 = (through_pixel[0] / context.width as f32) * 2.0 * std::f32::consts::PI;
                let s_y: f32 = (through_pixel[1] / context.height as f32) * std::f32::consts::PI;

                let eye: MatVec<3> = context.eye.clone();
                let forward: MatVec<3> = context.forward.clone().normalize();
                let up: MatVec<3> = context.up.normalize();
                let right: MatVec<3> = forward.cross(&up).normalize();

                let direction = (s_y.cos() * (s_x.cos() * right + s_x.sin() * up) + s_y.sin() * forward).normalize();

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
}