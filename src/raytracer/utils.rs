use std::convert::Infallible;

use crate::raytracer::MatVec;

use super::{Light, LightResidual};


pub fn compute_total_light(ilumination_sources: Vec<LightResidual>) -> Light {
    todo!("compute_total_light, Not yet implemented");
    // let mut total_light: Light = MatVec::new(vec![0.0, 0.0, 0.0]);
    // for light in ilumination_sources {
    //     total_light.color = total_light.color + light.color;
    //     total_light.intensity += light.intensity;
    // }
    // total_light
}

pub fn lambert(base_color: Light, ilumination_sources: Vec<LightResidual>) -> Light {
    todo!("lambert, Not yet implemented");
}

pub fn round_precision(value: f32, precision: f32) -> f32 {
    let rounded_value = (value / precision).round() * precision;
    if (value - rounded_value).abs() < precision {
        rounded_value
    } else {
        value
    }
}

#[inline(always)]
pub fn fuzzy_eq(a: f32, b: f32, epsilon: f32) -> bool {
    (a - b).abs() < epsilon
}