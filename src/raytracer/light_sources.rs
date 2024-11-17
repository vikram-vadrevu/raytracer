use super::{MatVec, RGBA};
use super::ray::Ray;
use super::scene::LightSource;

pub struct Sun {
    pub position: MatVec,
    pub color: RGBA,
}