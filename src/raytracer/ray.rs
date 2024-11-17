use super::{light_sources, CameraState, Intersection, IntersectionPayload, MatVec, ProjectionType, RGBA};
use super::scene::{LightSource, Scene, SceneObject};

#[derive(Debug)]
pub struct Ray {

    pub origin: MatVec,
    pub direction: MatVec,

}

impl Ray {
    
        pub fn new(origin: MatVec, direction: MatVec) -> Ray {
            Ray {
                origin,
                direction,
            }
        }

        // through_pixel is a 2 element vector representing the NON-NORMALIZED
        // pixel coordinates. All conversions and projections should be applied
        // here.
        pub fn generate_primary_ray(through_pixel: MatVec, context: &CameraState) -> Ray {
            match context.projection {
                ProjectionType::FLAT => {
                    let s_x: f32 = (2.0*through_pixel[0] - context.width as f32) / u32::max(context.width, context.height) as f32;
                    let s_y: f32 = (context.height as f32 - 2.0*through_pixel[1]) / u32::max(context.width, context.height) as f32;
                    let eye: MatVec = context.eye.clone();
                    let forward: MatVec = context.forward.clone();
                    let up: MatVec = context.up.normalize();
                    let right: MatVec = forward.cross(&up).normalize();
                    Ray::new(eye, forward + s_x*right + s_y*up)
                },
                _ => todo!("Projection type {:?} is not yet supported", context.projection),
            }
        }

        pub fn generate_light_ray(intersection: &Intersection, light_source: &Box<dyn LightSource>) -> Ray {
            let origin = intersection.point.clone();
            let dir = light_source.compute_inverse_direction(&origin);

            Ray::new(origin, dir)
        }

        pub fn refract(&self, normal: &MatVec, ior: f32) -> MatVec {
            let cos_i: f32 = -self.direction.dot(normal.clone());
            let sin_t2: f32 = ior.powi(2) * (1.0 - cos_i.powi(2));
            if sin_t2 > 1.0 {
                return MatVec::new(vec![0.0, 0.0, 0.0]);
            }
            let cos_t: f32 = (1.0 - sin_t2).sqrt();
            return ior * self.direction.clone() + (ior*cos_i - cos_t) * normal.clone();
        }

        pub fn inverse(&self) -> Ray {
            Ray {
                origin: self.direction.clone(),
                direction: self.origin.clone(),
            }
        }

        // pub fn trace_ray(&self, scene: &Scene, bounce_limit: u32) -> RGBA {
        //     // cast primary ray
        //     for shape in &scene.shapes {
        //         let intersection: IntersectionPayload = shape.intersect(self);
        //         if intersection.is_some() {
        //             let intersection: Intersection = intersection.unwrap();
        //             let normal: MatVec = shape.normal(&intersection.point);
        //             let color: MatVec = shape.color_at(&intersection.point);
        //             let light: MatVec = shape.apply_light_transform(&normal);
        //             let dir: MatVec = shape.apply_dir_transform(&normal);
        //             // let residual: RGBA = self.trace_ray(scene, bounce_limit - 1);
        //             // return color + light + dir + residual;
        //             return color + light + dir;
        //         }
        //     }
        //     return MatVec::new(vec![0.0, 0.0, 0.0]);
        // }

        // // represents a trace to a single intersection point
        // pub fn trace_single_ray(&self, scene: &Scene) -> RGBA {

        //     let mut color: MatVec = MatVec::new(vec![0.0, 0.0, 0.0]);
        //     let mut object_intersections: Vec<IntersectionPayload> = Vec::new();

        //     for shape in &scene.shapes {

        //         let intersection: IntersectionPayload = shape.intersect(self);
        //         if intersection.is_some() {
        //             object_intersections.push(intersection);
        //         }

        //     }
        //     // // TODO: replace with min heap
        //     // for intersection in object_intersections {
        //     //     // find minimum distance
        //     // }
        //     return MatVec::new(vec![0.0, 0.0, 0.0]);
        // }
}