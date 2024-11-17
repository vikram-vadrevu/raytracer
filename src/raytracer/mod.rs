use std::ops::{Add, Sub, Mul, Index};
use std::fmt::Debug;
use ray::Ray;
// For now MatVec represents a 'Mathematical Vector'
// In needed, I will change this to be a 'Matrix Vector',
// ie a matrix/tensor


// #[derive(Debug)]
pub struct MatVec {
    data: Vec<f32>,
}

impl Debug for MatVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, value) in self.data.iter().enumerate() {
            writeln!(f, "data[{}]: {}", i, value)?;
        }
        Ok(())
    }
}

impl Clone for MatVec {
    fn clone(&self) -> Self {
        let mut new_vec: Vec<f32> = Vec::new();
        for i in 0..self.data.len() {
            new_vec.push(self.data[i]);
        }
        MatVec {
            data: new_vec,
        }
    }
}

impl MatVec {

    pub fn new(data: Vec<f32>) -> MatVec {
        MatVec {
            data,
        }
    }

    pub fn get(&self, i: usize) -> &f32 {
        &self.data[i]
    }

    pub fn set(&mut self, i: usize, val: f32) {
        self.data[i] = val;
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn dot(&self, other: MatVec) -> f32 {
        assert_eq!(self.len(), other.len(), "Vectors must be of the same length");
        let mut sum = 0.0;
        for i in 0..self.len() {
            sum += self.get(i) * other.get(i);
        }
        sum
    }

    fn _add(self, other: MatVec) -> MatVec {
        assert_eq!(self.len(), other.len(), "Vectors must be of the same length");
        let mut new_data = Vec::new();
        for i in 0..self.len() {
            new_data.push(self.get(i) + other.get(i));
        }
        MatVec::new(new_data)
    }

    fn _sub(&self, other: MatVec) -> MatVec {
        assert_eq!(self.len(), other.len(), "Vectors must be of the same length");
        let mut new_data = Vec::new();
        for i in 0..self.len() {
            new_data.push(self.get(i) - other.get(i));
        }
        MatVec::new(new_data)
    }

    pub fn scale(&self, scalar: f32) -> MatVec {
        let mut new_data = Vec::new();
        for i in 0..self.len() {
            new_data.push(self.get(i) * scalar);
        }
        MatVec::new(new_data)
    }

    pub fn normalize(&self) -> MatVec {
        let mut new_data = Vec::new();
        let mut sum = 0.0;
        for i in 0..self.len() {
            sum += self.get(i) * self.get(i);
        }
        let mag = sum.sqrt();
        for i in 0..self.len() {
            new_data.push(self.get(i) / mag);
        }
        MatVec::new(new_data)
    }

    pub fn cross(&self, other: &MatVec) -> MatVec {
        if self.len() == 3 {
            return self._cross3x3(other);
        }
        todo!("MatVec::cross, Not yet implemented for vectors of length {}", self.len());
    }

    fn _cross3x3(&self, other: &MatVec) -> MatVec {
        assert!(self.len() == other.len(), "MatVec::cross, Size missmatch");
        let x = self.get(1) * other.get(2) - self.get(2) * other.get(1);
        let y = self.get(2) * other.get(0) - self.get(0) * other.get(2);
        let z = self.get(0) * other.get(1) - self.get(1) * other.get(0);
        MatVec::new(vec![x, y, z])
    }

    pub fn clip_to_u8(&self) -> Vec<u8> {
        let mut clipped: Vec<u8> = Vec::new();
        for i in 0..self.len() {
            let val = self.get(i).max(0.0).min(255.0);
            clipped.push(val as u8);
        }
        clipped
    }

}

impl Add for MatVec {
    type Output = MatVec;

    fn add(self, other: MatVec) -> MatVec {
        self._add(other)
    }
}

impl Sub for MatVec {
    type Output = MatVec;

    fn sub(self, other: MatVec) -> MatVec {
        self._sub(other)
    }
}

// M * k
impl Mul<f32> for MatVec {
    type Output = MatVec;

    fn mul(self, scalar: f32) -> MatVec {
        self.scale(scalar)
    }
}

// k * M
impl Mul<MatVec> for f32 {
    type Output = MatVec;

    fn mul(self, vec: MatVec) -> MatVec {
        vec.scale(self)
    }
}

// M * M (dot product)
impl Mul<MatVec> for MatVec {
    type Output = f32;

    fn mul(self, other: MatVec) -> f32 {
        self.dot(other)
    }
}


impl Index<usize> for MatVec {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
    }
}


pub struct Intersection {

    pub shape_id: Option<usize>, // Index of the shape in the scene
    pub point: MatVec,   // Intersection point in the world coordinate frame
    pub normal: MatVec,  // Normals with respect to the object in the world coordinate frame
    pub distance: f32,   // Distance from the ray origin to the intersection point
    pub residual: bool,  // Indicates whether or not the intersection will result in a residual

}

pub type IntersectionPayload = Option<Intersection>;
pub type RGBA = MatVec;
pub type Light = MatVec;

pub struct InputState {

    color: MatVec,
    // texcoord: MatVec,
    // texture: String,
    // roughness: f32,
    // shininess: f32,
    // transparency: f32,
    // index_of_refraction: f32,

}

impl InputState {

    pub fn new() -> InputState {
        InputState {
            color: MatVec::new(vec![1.0, 1.0, 1.0]),
            // texcoord: MatVec::new(vec![0.0, 0.0]),
            // texture: String::from(""),
            // roughness: 0.0,
            // shininess: 0.0,
            // transparency: 0.0,
            // index_of_refraction: 0.0,
        }
    }

}

#[derive(Debug)]
pub enum ProjectionType {
    FLAT,
    FISHEYE,
    PANORAMIC,
    DOF
}

pub struct CameraState {

    pub width: u32,
    pub height: u32,
    pub position: MatVec,
    pub forward: MatVec,
    pub up: MatVec,
    pub eye: MatVec,
    pub exposure: Option<f32>,
    pub projection: ProjectionType,

}

impl CameraState {

    pub fn new(width: u32, height: u32) -> CameraState {
        CameraState {
            width,
            height,
            position: MatVec::new(vec![0.0, 0.0, 0.0]),
            forward: MatVec::new(vec![0.0, 0.0, 1.0]),
            up: MatVec::new(vec![0.0, 1.0, 0.0]),
            eye: MatVec::new(vec![0.0, 0.0, 0.0]),
            exposure: None,
            projection: ProjectionType::FLAT,
        }
    }

}

pub struct LightResidual {
    pub source_id: Option<usize>,
    pub color: RGBA,
    pub intensity: f32,
    pub direction: Ray,
}

impl LightResidual {
    // default light residual represents a shadow at a point
    pub fn new() -> LightResidual {
        LightResidual {
            source_id: None,
            color: MatVec::new(vec![0.0, 0.0, 0.0]),
            intensity: 0.0,
            direction: Ray::new(MatVec::new(vec![0.0, 0.0, 0.0]), MatVec::new(vec![0.0, 0.0, 0.0])),
        }
    }
}

// Export internal modules
pub mod raytracer;
pub mod ray;
pub mod scene;
pub mod shapes;
pub mod light_sources;
pub mod utils;
// pub mod materials;