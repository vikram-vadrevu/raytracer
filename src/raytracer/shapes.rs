use crate::raytracer::{Color, InputState, Intersection, IntersectionPayload, MatVec};
use crate::raytracer::scene::SceneObject;
use crate::raytracer::ray::Ray;
use crate::raytracer::utils;
use crate::raytracer::texture::Texture2d;

/// Represents a sphere in 3D space.
pub struct Sphere {
    pub center: MatVec<3>,
    pub radius: f32,
    pub color: Color,
    pub texture: Option<Texture2d>,
    pub shininess: Option<Vec<f32>>,
    pub transparency: Option<Vec<f32>>,
    pub roughness: f32,
    // pub material: Material,
}

impl Sphere {

    pub fn new(center: MatVec<3>, radius: f32, context: &InputState) -> Sphere {

        println!("Making sphere with center: {:?}, radius: {:?}, color: {:?}", center, radius, context.color);

        // Function assumes that the texture path in the context is valid and exists
        let texture: Option<Texture2d> = match context.texture.as_str() {
            "none" => None,
            _ => Some(Texture2d::new(&context.texture)),
        };

        let shininess: Option<Vec<f32>> = match context.shininess.is_empty() {
            true => None,
            false => {
                if context.shininess.len() == 3 {
                    Some(context.shininess.clone())
                } else if context.shininess.len() == 1 {
                    Some(vec![context.shininess[0]; 3])
                } else {
                    None
                }
            }
        };

        let transparency: Option<Vec<f32>> = match context.transparency.is_empty() {
            true => None,
            false => {
            if context.transparency.len() == 3 {
                Some(context.transparency.clone())
            } else if context.transparency.len() == 1 {
                Some(vec![context.transparency[0]; 3])
            } else {
                None
            }
            }
        };



        Sphere {
            center,
            radius,
            color: context.color.clone(),
            texture,
            shininess,
            transparency,
            roughness: context.roughness,
            // material,
        }

    }

}

impl SceneObject for Sphere {

    fn propagate(&self, incident: &Ray) -> Ray {
        // Move the origin slightly outside the sphere along the ray direction
        let offset = 0.001; // Small offset to move the origin outside the sphere
        let direction_to_center = (self.center.clone() - incident.origin.clone()).normalize();
        let new_origin = incident.origin.clone() + (self.radius + offset) * direction_to_center;
        Ray {
            origin: new_origin,
            direction: incident.direction.clone(),
        }
    }

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
        let mut normal: MatVec<3> = (intersection_point.clone() - self.center.clone()).normalize();

        normal = normal.perturb(0.5_f32, self.roughness).normalize();

        Some(Intersection {
            shape_id: None,
            point: intersection_point,
            normal: normal,
            distance: t,
            residual: false,
        })

    }

    fn color_at(&self, point: &MatVec<3>) -> Color {

        match self.texture {
            None => self.color.clone(),
            Some(ref texture) => {
                let uv_coord: MatVec<2> = utils::spherical_world_to_uv(point, &self.center, self.radius);
                texture.sample(uv_coord)
            },
        }
    }

    fn shininess(&self) -> Option<Vec<f32>> {
        self.shininess.clone()
    }

    fn transparency(&self) -> Option<Vec<f32>> {
        self.transparency.clone()
    }

}

/// Represents a plane in 3D space.
#[allow(non_snake_case)]
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

    fn propagate(&self, incident: &Ray) -> Ray {
        todo!("not implemented yet");
    }
    
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

}

/// Represents a triangle in 3D space.
pub struct Triangle {
    pub verticies: [MatVec<3>; 3],
    pub color: Color,
    pub texture: Option<Texture2d>,
    pub texcoords: Option<Vec<MatVec<2>>>,
    roughness: f32,
}

impl Triangle {
    pub fn new(indices: Vec<i32>, context: &InputState) -> Triangle {
        assert_eq!(indices.len(), 3, "Triangle must have exactly 3 vertices");
        println!("Making triangle with vertices: {:?}, color: {:?}", indices, context.color);

        let verticies: Vec<MatVec<3>> = indices.iter().map(|&i| {
            if i < 0 {
                context.verticies[(context.verticies.len() as i32 + i) as usize].clone()
            } else {
                context.verticies[(i - 1) as usize].clone()
            }
        }).collect();

        let texcoords: Option<Vec<MatVec<2>>> = if context.texture != "none" && !context.texcoords.is_empty() {
            Some(indices.iter().map(|&i| {
            if i < 0 {
                context.texcoords[(context.texcoords.len() as i32 + i) as usize].clone()
            } else {
                context.texcoords[(i - 1) as usize].clone()
            }
            }).collect())
        } else {
            None
        };

        let texture: Option<Texture2d> = match context.texture.as_str() {
            "none" => None,
            _ => {
                Some(Texture2d::new(&context.texture))
            },
        };

        Triangle {
            verticies: [verticies[0].clone(), verticies[1].clone(), verticies[2].clone()],
            color: context.color.clone(),
            texture,
            texcoords,
            roughness: 0.0_f32,
        }
    }

    // fn uv_at(&self, point: &MatVec<3>) -> MatVec<2> {
    //     let texcoords = self.texcoords.as_ref().unwrap();
    //     let v0 = self.vertices[1].clone() - self.vertices[0].clone();
    //     let v1 = self.vertices[2].clone() - self.vertices[0].clone();
    //     let v2 = point.clone() - self.vertices[0].clone();
    
    //     // Compute dot products for the barycentric coordinates
    //     let d00 = v0.dot(v0.clone());
    //     let d01 = v0.dot(v1.clone());
    //     let d11 = v1.dot(v1.clone());
    //     let d20 = v2.dot(v0.clone());
    //     let d21 = v2.dot(v1.clone());
    
    //     // Compute the denominator of the barycentric coordinates
    //     let denom = d00 * d11 - d01 * d01;
    
    //     if denom.abs() < 1e-6 {
    //         panic!("Triangle vertices are degenerate or too close together!");
    //     }
    
    //     // Barycentric coordinates
    //     let v = (d11 * d20 - d01 * d21) / denom;
    //     let w = (d00 * d21 - d01 * d20) / denom;
    //     let u = 1.0 - v - w;
    
    //     // Interpolate the UV coordinates using barycentric weights
    //     texcoords[0].clone() * u + texcoords[1].clone() * v + texcoords[2].clone() * w
    // }
    

}

impl SceneObject for Triangle {


    fn propagate(&self, incident: &Ray) -> Ray {
        todo!("not implemented yet");
    }

    fn intersect(&self, ray: &Ray) -> IntersectionPayload {
            
            let edge1: MatVec<3> = self.verticies[1].clone() - self.verticies[0].clone();
            let edge2: MatVec<3> = self.verticies[2].clone() - self.verticies[0].clone();
    
            let h: MatVec<3> = ray.direction.clone().cross(&edge2);
            let a: f32 = edge1.clone().dot(h.clone());
    
            if a.abs() < 0.0001 {
                return None;
            }
    
            let f: f32 = 1.0 / a;
            let s: MatVec<3> = ray.origin.clone() - self.verticies[0].clone();
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

            normal = normal.perturb(0.01, self.roughness).normalize();
    
            Some(Intersection {
                shape_id: None,
                point: intersection_point,
                normal,
                distance: t,
                residual: false,
            })
            
    }

    fn color_at(&self, point: &MatVec<3>) -> Color {
        match self.texture {

            None => self.color.clone(),

            Some(ref texture) => {
                // todo!("Texture mapping for triangles doesnt work yet");
                // let uv_coord: MatVec<2> = self.uv_at(point);
                let uv_coord: MatVec<2> = utils::barycentric_uv(point, self.verticies.to_vec(), self.texcoords.as_ref().unwrap().clone());
                texture.sample(uv_coord)
            },
            
        }

    }

}