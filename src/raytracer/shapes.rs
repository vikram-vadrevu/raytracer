use crate::raytracer::{MatVec, InputState, Intersection, IntersectionPayload, RGBA};
use crate::raytracer::scene::SceneObject;
use crate::raytracer::ray::Ray;
pub struct Sphere {
    pub center: MatVec,
    pub radius: f32,
    // pub material: Material,
}

impl Sphere {
    pub fn new(center: MatVec, radius: f32, context: &InputState) -> Sphere {
        Sphere {
            center,
            radius,
            // material,
        }
    }
}

impl SceneObject for Sphere {

    fn intersect(&self, ray: &Ray) -> IntersectionPayload {

        let l = self.center.clone() - ray.origin.clone();
        let tca = l.clone() * ray.direction.clone();
        if tca < 0.0 {
            return None;
        }
        let d2 = l.dot(l.clone()) - tca * tca;
        let r2 = self.radius * self.radius;
        if d2 > r2 {
            return None;
        }
        let thc = (r2 - d2).sqrt();
        let t0 = tca - thc;
        let t1 = tca + thc;
        if t0 < 0.0 && t1 < 0.0 {
            return None;
        }
        let t = if t0 < t1 { t0 } else { t1 };
        let point = ray.origin.clone() + ray.direction.clone().scale(t);
        let normal = (point.clone() - self.center.clone()).normalize();

        Some(Intersection {
            shape_id : None,
            point : point,
            normal : normal,
            distance: t,
            residual: false,
        })

    }

    fn normal(&self, point: &MatVec) -> MatVec {
        // (point - self.center).normalize()
        MatVec::new(vec![0.0])
    }
    fn color_at(&self, point: &MatVec) -> RGBA {todo!("color_at not implemented")}
    fn apply_dir_transform(&self, dir: &MatVec) -> MatVec {todo!("apply_dir_transform not implemented")}
    fn apply_light_transform(&self, light: &MatVec) -> MatVec {todo!("apply_light_transform not implemented")}

}