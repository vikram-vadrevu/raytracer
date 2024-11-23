use crate::raytracer::{MatVec, InputState, Intersection, IntersectionPayload, RGBA};
use crate::raytracer::scene::SceneObject;
use crate::raytracer::ray::Ray;
use crate::raytracer::utils;

pub struct Sphere {
    pub center: MatVec,
    pub radius: f32,
    pub color: RGBA,
    // pub material: Material,
}

impl Sphere {
    pub fn new(center: MatVec, radius: f32, context: &InputState) -> Sphere {
        println!("Making sphere with center: {:?}, radius: {:?}, color: {:?}", center, radius, context.color);
        Sphere {
            center,
            radius,
            color: context.color.clone(),
            // material,
        }
    }
}

impl SceneObject for Sphere {

    fn intersect(&self, ray: &Ray) -> IntersectionPayload {

        // println!("ray: {:?}", ray);
        // println!("center: {:?}", self.center);
        // println!("radius: {:?}", self.radius);

        let inside: bool = (self.center.clone() - ray.origin.clone()).magnitude() < self.radius;
        
        // println!("inside: {}", inside);

        let tc: f32 = ((self.center.clone() - ray.origin.clone()).dot(ray.direction.clone())) / ray.direction.clone().magnitude();

        if !inside && tc < 0.0 {
            return None;
        }

        let d: f32 = (ray.origin.clone() + (tc * ray.direction.clone()) - self.center.clone()).magnitude();
        
        let d2: f32 = utils::round_precision(f32::powi(d, 2), 0.005);

        if !inside && f32::powi(self.radius, 2) <= d2 {
            return None;
        }

        // if !fuzzy_eq(f32::powi(self.radius, 2), d2, 0.005f32) {
        //     return None;
        // }

        let t_offset: f32 = f32::sqrt(f32::powi(self.radius, 2) - d2) / ray.direction.clone().magnitude();

        let t: f32 = if inside {
            tc + t_offset
        } else {
            tc - t_offset
        };

        let intersection_point: MatVec = ray.origin.clone() + t * ray.direction.clone();
        let normal: MatVec = (intersection_point.clone() - self.center.clone()).normalize();

        Some(Intersection {
            shape_id: None,
            point: intersection_point,
            normal: normal,
            distance: t,
            residual: false,
        })
        // let l = self.center.clone() - ray.origin.clone();
        // let tca = l.clone() * ray.direction.clone();
        // if tca < 0.0 {
        //     return None;
        // }
        // let d2 = l.dot(l.clone()) - tca * tca;
        // let r2 = self.radius * self.radius;
        // if d2 > r2 {
        //     return None;
        // }
        // let thc = (r2 - d2).sqrt();
        // let t0 = tca - thc;
        // let t1 = tca + thc;
        // if t0 < 0.0 && t1 < 0.0 {
        //     return None;
        // }
        // let t = if t0 < t1 { t0 } else { t1 };
        // let point = ray.origin.clone() + ray.direction.clone().scale(t);
        // let normal = (point.clone() - self.center.clone()).normalize();
        //
        // Some(Intersection {
        //     shape_id : None,
        //     point : point,
        //     normal : normal,
        //     distance: t,
        //     residual: false,
        // })

    }

    fn normal(&self, point: &MatVec) -> MatVec {
        // (point - self.center).normalize()
        MatVec::new(vec![0.0])
    }
    fn color_at(&self, point: &MatVec) -> RGBA {
        self.color.clone()
    }
    fn apply_dir_transform(&self, dir: &MatVec) -> MatVec {todo!("apply_dir_transform not implemented")}
    fn apply_light_transform(&self, light: &MatVec) -> MatVec {todo!("apply_light_transform not implemented")}

}