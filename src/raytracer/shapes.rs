use crate::raytracer::{MatVec, InputState, Intersection, IntersectionPayload, Color};
use crate::raytracer::scene::SceneObject;
use crate::raytracer::ray::Ray;
use crate::raytracer::utils;

pub struct Sphere {
    pub center: MatVec<3>,
    pub radius: f32,
    pub color: Color,
    // pub material: Material,
}

impl Sphere {
    pub fn new(center: MatVec<3>, radius: f32, context: &InputState) -> Sphere {
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

        let intersection_point: MatVec<3> = ray.origin.clone() + t * ray.direction.clone();
        let normal: MatVec<3> = (intersection_point.clone() - self.center.clone()).normalize();

        Some(Intersection {
            shape_id: None,
            point: intersection_point,
            normal: normal,
            distance: t,
            residual: false,
        })

    }

    fn color_at(&self, _point: &MatVec<3>) -> Color {

        self.color.clone()

    }

    // fn normal(&self, point: &MatVec<3>) -> MatVec<3> {
    //     // (point - self.center).normalize()
    //     MatVec::new(vec![0.0])
    // }

    // fn apply_dir_transform(&self, dir: &MatVec<3>) -> MatVec<3> {todo!("apply_dir_transform not implemented")}
    // fn apply_light_transform(&self, light: &MatVec<3>) -> MatVec<3> {todo!("apply_light_transform not implemented")}

}

pub struct Plane {
    pub normal: MatVec<3>,
    pub D: f32,
    pub color: Color,
}

impl Plane {
    pub fn new(coeffs: MatVec<4>, context: &InputState) -> Plane {
        println!("Making plane with coeffs: {:?}, color: {:?}", coeffs, context.color);
        Plane {
            normal: MatVec::new(vec![*coeffs.get(0), *coeffs.get(1), *coeffs.get(2)]).normalize(),
            D: *coeffs.get(3),
            color: context.color.clone(),
        }
    }
}

impl SceneObject for Plane {
    
    fn intersect(&self, ray: &Ray) -> IntersectionPayload {

        let denom: f32 = self.normal.clone().dot(ray.direction.clone());

        if denom.abs() < 0.0001 {
            return None;
        }

        let t: f32 = -(self.normal.clone().dot(ray.origin.clone()) + self.D) / denom;

        if t < 0.0 {
            return None;
        }

        let intersection_point: MatVec<3> = ray.origin.clone() + t * ray.direction.clone();
        
        let normal: MatVec<3> = self.normal.clone();

        Some(Intersection {
            shape_id: None,
            point: intersection_point,
            normal,
            distance: t,
            residual: false,
        })

    }

    fn color_at(&self, _point: &MatVec<3>) -> Color {

        self.color.clone()

    }

    // fn normal(&self, point: &MatVec<3>) -> MatVec<3> {
    //     // self.coeffs.get(0..3).normalize()
    //     MatVec::new(vec![0.0])
    // }

    // fn apply_dir_transform(&self, dir: &MatVec<3>) -> MatVec<3> {todo!("apply_dir_transform not implemented")}
    // fn apply_light_transform(&self, light: &MatVec<3>) -> MatVec<3> {todo!("apply_light_transform not implemented")}
}

pub struct Triangle {
    pub vertices: [MatVec<3>; 3],
    pub color: Color,
}

impl Triangle {
    pub fn new(vertices: Vec<MatVec<3>>, context: &InputState) -> Triangle {
        assert_eq!(vertices.len(), 3, "Triangle must have exactly 3 vertices");
        println!("Making triangle with vertices: {:?}, color: {:?}", vertices, context.color);
        Triangle {
            vertices: [vertices[0].clone(), vertices[1].clone(), vertices[2].clone()],
            color: context.color.clone(),
        }
    }
}

impl SceneObject for Triangle {

    fn intersect(&self, ray: &Ray) -> IntersectionPayload {
            
            let edge1: MatVec<3> = self.vertices[1].clone() - self.vertices[0].clone();
            let edge2: MatVec<3> = self.vertices[2].clone() - self.vertices[0].clone();
    
            let h: MatVec<3> = ray.direction.clone().cross(&edge2);
            let a: f32 = edge1.clone().dot(h.clone());
    
            if a.abs() < 0.0001 {
                return None;
            }
    
            let f: f32 = 1.0 / a;
            let s: MatVec<3> = ray.origin.clone() - self.vertices[0].clone();
            let u: f32 = f * s.clone().dot(h.clone());
    
            if u < 0.0 || u > 1.0 {
                return None;
            }
    
            let q: MatVec<3> = s.clone().cross(&edge1);
            let v: f32 = f * ray.direction.clone().dot(q.clone());
    
            if v < 0.0 || u + v > 1.0 {
                return None;
            }
    
            let t: f32 = f * edge2.clone().dot(q.clone());
    
            if t < 0.0001 {
                return None;
            }
    
            let intersection_point: MatVec<3> = ray.origin.clone() + t * ray.direction.clone();
            let mut normal: MatVec<3> = edge1.clone().cross(&edge2).normalize();
            if normal.dot(ray.direction.clone()) > 0.0 {
                normal = -1.0f32 * normal;
            }
    
            Some(Intersection {
                shape_id: None,
                point: intersection_point,
                normal,
                distance: t,
                residual: false,
            })
            
    }

    fn color_at(&self, _point: &MatVec<3>) -> Color {

        self.color.clone()

    }

}