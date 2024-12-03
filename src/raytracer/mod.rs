use std::ops::{Add, Sub, Mul, Index};
use std::fmt::Debug;

// For now MatVec represents a 'Mathematical Vector'
// In needed, I will change this to be a 'Matrix Vector',
// ie a matrix/tensor

/// `MatVec` or 'Mathematical Vector' is a generic wrapper around
/// f32 arrays with mathematical operations defined.
/// This type is the funamental building block for the raytracer.
pub struct MatVec<const N: usize> {
    data: [f32; N],
}

// Method Implementations for MatVec
impl<const N: usize> MatVec<N> {

    pub fn new(data: Vec<f32>) -> MatVec<N> {
        assert_eq!(data.len(), N, "MatVec::new, Size missmatch");
        let mut new_data = [0.0f32; N];
        for i in 0..N {
            new_data[i] = data[i];
        }
        MatVec::<N> {
            data: new_data,
        }
    }

    pub fn get(&self, i: usize) -> &f32 {
        assert!(i < N, "MatVec::get, Index out of bounds");
        &self.data[i]
    }

    pub fn set(&mut self, i: usize, val: f32) {
        self.data[i] = val;
    }

    pub fn len(&self) -> usize {
        // self.data.len()
        N
    }

    pub fn dot(&self, other: MatVec::<N>) -> f32 {
        let mut sum = 0.0;
        for i in 0..N {
            sum += self.get(i) * other.get(i);
        }
        sum
    }

    fn _add(self, other: MatVec<N>) -> MatVec<N> {
        // assert_eq!(self.len(), other.len(), "Vectors must be of the same length");
        let mut new_data = Vec::new();
        for i in 0..N {
            new_data.push(self.get(i) + other.get(i));
        }
        MatVec::new(new_data)
    }

    fn _sub(&self, other: MatVec<N>) -> MatVec<N> {
        // assert_eq!(self.len(), other.len(), "Vectors must be of the same length");
        let mut new_data = Vec::new();
        for i in 0..N {
            new_data.push(self.get(i) - other.get(i));
        }
        MatVec::new(new_data)
    }

    pub fn scale(&self, scalar: f32) -> MatVec<N> {
        let mut new_data = Vec::new();
        for i in 0..N {
            new_data.push(self.get(i) * scalar);
        }
        MatVec::new(new_data)
    }

    pub fn normalize(&self) -> MatVec<N> {
        let mut new_data = Vec::new();
        let mut sum = 0.0;
        for i in 0..N {
            sum += self.get(i) * self.get(i);
        }
        let mag = sum.sqrt();
        for i in 0..N {
            new_data.push(self.get(i) / mag);
        }
        MatVec::new(new_data)
    }

    pub fn magnitude(&self) -> f32 {
        let mut sum = 0.0;
        for i in 0..N {
            sum += self.get(i) * self.get(i);
        }
        sum.sqrt()
    }

    pub fn cross(&self, other: &MatVec<N>) -> MatVec<N> {
        if N == 3 {
            return self._cross3x3(other);
        }
        todo!("MatVec::cross, Not yet implemented for vectors of length {}", self.len());
    }

    // TODO: Need to remove this function from here since it is only valid for 3D vectors
    fn _cross3x3(&self, other: &MatVec<N>) -> MatVec <N>{
        assert!(self.len() == other.len(), "MatVec::cross, Size missmatch");
        let x = self.get(1) * other.get(2) - self.get(2) * other.get(1);
        let y = self.get(2) * other.get(0) - self.get(0) * other.get(2);
        let z = self.get(0) * other.get(1) - self.get(1) * other.get(0);
        MatVec::new(vec![x, y, z])
    }

    pub fn clip_to_u8(&self) -> Vec<u8> {
        let mut clipped: Vec<u8> = Vec::new();
        for i in 0..N {
            let val = self.get(i).max(0.0).min(255.0);
            clipped.push(val as u8);
        }
        clipped
    }

    // TODO: Find a new home for this funcion since it is only valid for 4d vectors
    // Computes the rgba vector from a MatVec with values from 0.0 to 1.0
    pub fn to_rgba(&self) -> image::Rgba<u8> {
        if self.len() == 4 {
            return image::Rgba([(*self.get(0) * 255.0) as u8,
                                (*self.get(1) * 255.0) as u8,
                                (*self.get(2) * 255.0) as u8,
                                (*self.get(3) * 255.0) as u8]);
        }
        else if self.len() == 3 {
            return image::Rgba([(*self.get(0) * 255.0) as u8,
                                (*self.get(1) * 255.0) as u8,
                                (*self.get(2) * 255.0) as u8,
                                255 as u8]);
        }
        else {
            panic!("MatVec::to_rgba, Invalid length for MatVec");
        }
    }

}

// Implementations for standard traits and operators on MatVec types

impl<const N: usize> Debug for MatVec<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, value) in self.data.iter().enumerate() {
            writeln!(f, "data[{}]: {}", i, value)?;
        }
        Ok(())
    }
}

impl<const N: usize> Clone for MatVec<N> {
    fn clone(&self) -> Self {
        let mut new_data = [0.0f32; N];
        for i in 0..N {
            new_data[i] = self.data[i];
        }
        MatVec::<N> {
            data: new_data,
        }
    }
}

impl<const N: usize> Copy for MatVec<N> {}

impl<const N: usize> Add for MatVec<N> {
    type Output = MatVec<N>;

    fn add(self, other: MatVec<N>) -> MatVec<N> {
        self._add(other)
    }
}

impl<const N: usize> Sub for MatVec<N> {
    type Output = MatVec<N>;

    fn sub(self, other: MatVec<N>) -> MatVec<N> {
        self._sub(other)
    }
}

/// Multiplication of a MatVec by a scalar.
/// Represents M * k
impl<const N: usize> Mul<f32> for MatVec<N> {
    type Output = MatVec<N>;

    fn mul(self, scalar: f32) -> MatVec<N> {
        self.scale(scalar)
    }
}

/// Multiplication of a scalar by a MatVec.
/// Represents k * M
impl<const N: usize> Mul<MatVec<N>> for f32 {
    type Output = MatVec<N>;

    fn mul(self, vec: MatVec<N>) -> MatVec<N> {
        vec.scale(self)
    }
}

// NOTE: In general, its is probably not the best idea to
// define the dot product as the standard multiplication for vector data types

// // M * M (dot product)
// impl<const N: usize> Mul<MatVec<N>> for MatVec<N> {
//     type Output = f32;
//
//     fn mul(self, other: MatVec<N>) -> f32 {
//         self.dot(other)
//     }
// }

impl<const N: usize> Index<usize> for MatVec<N> {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
    }
}

// Other Commonly used types and constructors

#[derive(Debug, Clone)]
pub struct Intersection {

    pub shape_id: Option<usize>, // Index of the shape in the scene
    pub point: MatVec<3>,     // Intersection point in the world coordinate frame
    pub normal: MatVec<3>,    // Normals with respect to the object in the world coordinate frame
    pub distance: f32,          // Distance from the ray origin to the intersection point
    pub residual: bool,         // Indicates whether or not the intersection will result in a residual

}

pub struct InputState {

    color: Color,
    texture: String,
    verticies: Vec<MatVec<3>>,
    texcoords: Vec<MatVec<2>>,
    // texture: String,
    // roughness: f32,
    shininess: Vec<f32>,
    // transparency: f32,
    // index_of_refraction: f32,

}

impl InputState {

    pub fn new() -> InputState {
        InputState {
            color: MatVec::new(vec![1.0, 1.0, 1.0]),
            texture: "none".to_string(),
            verticies: Vec::new(),
            texcoords: Vec::new(),
            shininess: Vec::new(),
        }
    }

}

#[derive(Debug)]
pub enum ProjectionType {

    FLAT,
    FISHEYE,
    PANORAMIC,
    // DOF,

}

pub struct CameraState {

    pub width: u32,
    pub height: u32,
    pub forward: MatVec<3>,
    pub up: MatVec<3>,
    pub eye: MatVec<3>,
    pub exposure: Option<f32>,
    pub projection: ProjectionType,

}

impl CameraState {

    pub fn new(width: u32, height: u32) -> CameraState {
        CameraState {
            width,
            height,
            forward: MatVec::new(vec![0.0, 0.0, -1.0]),
            up: MatVec::new(vec![0.0, 1.0, 0.0]),
            eye: MatVec::new(vec![0.0, 0.0, 0.0]),
            exposure: None,
            projection: ProjectionType::FLAT,
        }
    }

}

pub struct LightResidual {
    pub source_id: Option<usize>, // TODO: Will be used in reflections
    pub color: Color,
    pub intensity: f32,
    pub direction: MatVec<3>,
    pub normal: MatVec<3>,
}

impl LightResidual {
    // default light residual represents a shadow at a point
    pub fn new() -> LightResidual {
        LightResidual {
            source_id: None,
            color: MatVec::new(vec![0.0, 0.0, 0.0]),
            intensity: 0.0,
            // direction: Ray::new(MatVec::new(vec![0.0, 0.0, 0.0]), MatVec::new(vec![0.0, 0.0, 0.0])),
            direction: MatVec::new(vec![0.0, 0.0, 0.0]),
            normal: MatVec::new(vec![0.0,0.0,0.0]),
        }
    }
}

// Type aliases
pub type IntersectionPayload = Option<Intersection>;
pub type RGBA = MatVec<4>;
pub type Color = MatVec<3>;
pub type Light = MatVec<3>;

// Export internal modules
pub mod raytracer;
mod ray;
mod scene;
mod shapes;
mod light_sources;
mod utils;
mod texture;
mod material;