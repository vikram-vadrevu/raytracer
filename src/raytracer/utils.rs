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

/// Computes the color of a pixel based on the lambertian model (diffuse lighting).
/// Uses the color of the object at the pixel that is being rendered, and all of the
/// light sources in the scene, that do not have any objects between them and the
/// intersection point.
pub fn lambert(base_color: &Light, ilumination_sources: &Vec<LightResidual>) -> RGBA {

    let mut total: Color = MatVec::new(vec![0.0, 0.0, 0.0]);

    for source in ilumination_sources {

        let lambert: f32 = f32::max(source.normal.dot(source.direction.clone()), 0.0);
        let mut temp: MatVec<3> = source.intensity * lambert * source.color.clone();

        for i in 0..3 {

            temp.set(i, temp.get(i).clone() * base_color.get(i).clone());

        }

        total = total + temp.clone();

    }

    color_to_rgba(total, 1.0f32)
}

/// Rounds a value to the nearest multiple of a given precision.
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


/// Computes the gamma corrected value of a given color component
#[inline(always)]
pub fn gamma_correct(value: f32) -> f32 {
    if value <= 0.0031308 { 12.92 * value } else { 1.055 * value.powf(1.0 / 2.4) - 0.055 }
}

/// Converts a linear color to sRGB color space
#[allow(non_snake_case)]
pub fn sRGB(color: &RGBA) -> MatVec<4> {
    let mut to_return: Vec<f32> = Vec::new();
    for i in 0..3 {
        to_return.push(gamma_correct(color.get(i).clone()));
    }
    to_return.push(color.get(3).clone()); // Pass through the 4th element
    MatVec::new(to_return)
}

/// This function just passes through the color and sets the alpha value
/// to the given value.
/// This is essentially only used to convert Color (`MatVec<3>`) to RGBA (`MatVec<4>`)
#[inline(always)]
pub fn color_to_rgba(color: Color, alpha: f32) -> RGBA {
    MatVec::<4>::new(vec![*color.get(0),
                                  *color.get(1),
                                  *color.get(2), 
                                  alpha])
}

/// Applies an exponential exposure to a color, and passes through the alpha channel.
pub fn appy_exposure(color: &RGBA, exposure: f32) -> RGBA {

    let mut to_return: Vec<f32> = Vec::new();

    for i in 0..3 {

        to_return.push(1.0 - ( -color.get(i).clone() * exposure).exp());

    }

    to_return.push(color.get(3).clone()); // Pass through the alpha channel
    MatVec::new(to_return)

}

/// Computes the UV texture coordinates of a point on a sphere.
/// Given a point in 3D space (that belongs on the given sphere), and the center and radius of the sphere,
/// computes the UV coordinates of the point in normalized texture coordinates.
/// The point is first converted to a longitude and latitude, and then the UV coordinates are computed.
pub fn spherical_world_to_uv(point: &MatVec<3>, center: &MatVec<3>, radius: f32) -> MatVec<2> {
    let translated_point = point.clone() - center.clone();
    let phi: f32 = f32::atan2(*translated_point.get(2), *translated_point.get(0));
    let theta: f32 = f32::acos(*translated_point.get(1) / radius);

    let u: f32 = 1.0 - (phi + std::f32::consts::PI) / (2.0 * std::f32::consts::PI);
    let v: f32 = theta / std::f32::consts::PI;

    MatVec::new(vec![u, v])
}


/// Cmputes the UV texture coordinates of a point in a triangle.
/// Given a point in 3D space (that belongs on the given triangle), and the texcoords of the triangle,
/// computes the UV coordinates of the point in normalized texture coordinates.
pub fn barycentric_uv(point: &MatVec<3>, verticies: Vec<MatVec<3>>, texcoords: Vec<MatVec<2>>) -> MatVec<2> {
    let v0: MatVec<3> = verticies[1].clone() - verticies[0].clone();
    let v1: MatVec<3>  = verticies[2].clone() - verticies[0].clone();
    let v2: MatVec<3>  = point.clone() - verticies[0].clone();
    
    let d00 = v0.dot(v0.clone());
    let d01 = v0.dot(v1.clone());
    let d11 = v1.dot(v1.clone());
    let d20 = v2.dot(v0.clone());
    let d21 = v2.dot(v1.clone());
    
    let denom = d00 * d11 - d01 * d01;
    let v = (d11 * d20 - d01 * d21) / denom;
    let w = (d00 * d21 - d01 * d20) / denom;
    let u = 1.0 - v - w;
    
    let uv = texcoords[0].clone() * u + texcoords[1].clone() * v + texcoords[2].clone() * w;
    MatVec::new(vec![uv.get(0).clone(), uv.get(1).clone()])
}
