use crate::raytracer::MatVec;
use super::{Light, LightResidual, RGBA, Color};


// pub fn compute_total_light(ilumination_sources: Vec<LightResidual>) -> Light {
//     todo!("compute_total_light, Not yet implemented");
//     // let mut total_light: Light = MatVec::new(vec![0.0, 0.0, 0.0]);
//     // for light in ilumination_sources {
//     //     total_light.color = total_light.color + light.color;
//     //     total_light.intensity += light.intensity;
//     // }
//     // total_light
// }

pub fn lambert(base_color: &Light, ilumination_sources: &Vec<LightResidual>) -> RGBA {
    let mut total: Color = base_color.clone();
    for source in ilumination_sources {
        let lambert: f32 = f32::max(source.normal.dot(source.direction.direction.clone()), 0.0);
        let mut temp: MatVec<3> = lambert * source.color.clone();
        for i in 0..3 {
            // temp[i] = temp[i] * base_color[i];
            temp.set(i, temp.get(i).clone() * base_color.get(i).clone());
        }
        total = total + temp;
    }
    color_to_rgba(total, 1.0f32)
}

pub fn round_precision(value: f32, precision: f32) -> f32 {
    let rounded_value = (value / precision).round() * precision;
    if (value - rounded_value).abs() < precision {
        rounded_value
    } else {
        value
    }
}

// #[inline(always)]
// pub fn fuzzy_eq(a: f32, b: f32, epsilon: f32) -> bool {
//     (a - b).abs() < epsilon
// }

// #[inline(always)]
// pub fn gamma_correct(value: f32) -> f32 {
//     if value <= 0.0031308 {
//         12.92 * value
//     } else {
//         1.055 * value.powf(1.0 / 2.4) - 0.055
//     }
// }

// #[allow(non_snake_case)]
// pub fn sRGB(color: &RGBA) -> MatVec{
//     let mut to_return: Vec<f32> = Vec::new();
//     for i in 0..3 {
//         to_return.push(gamma_correct(color.get(i).clone()));
//     }
//     MatVec::new(to_return)
// }

pub fn color_to_rgba(color: Color, alpha: f32) -> RGBA {
    MatVec::<4>::new(vec![*color.get(0),
                                  *color.get(1),
                                  *color.get(2), 
                                  alpha])
}