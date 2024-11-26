use super::{scene, utils, CameraState, InputState, MatVec, RGBA, ProjectionType};
use super::ray::Ray;
use std::fs::File;
use std::io::{BufRead, BufReader};
use image::{ImageBuffer, RgbaImage};
use super::shapes::{*};
use super::light_sources::{*};

pub struct RayTracer {

    scene: scene::Scene,
    height: u32,
    width: u32,
    bounce_limit: u32,
    // other porperties
    input_state: InputState,
    image: RgbaImage,
    camera: CameraState,

}


impl RayTracer {

    pub fn new(height: u32, width: u32) -> RayTracer {
        let scene = scene::Scene::new();
        let default_bounce_limit = 1;
        RayTracer {
            scene,
            height,
            width,
            bounce_limit : default_bounce_limit,
            input_state : InputState::new(),
            image: ImageBuffer::new(width, height),
            camera : CameraState::new(width, height),
        }
    }

    pub fn render_from_file(file_path: &str) {
        println!("Rendering from file: {}", file_path);
        let file = File::open(file_path).expect("File not found");
        let reader = BufReader::new(file);

        // Read the lines into a vector of strings
        let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

        // // Parse the header line
        let header: &String = &lines[0];
        let header_parts: Vec<String> = header.split_whitespace().map(|s| s.to_string()).collect();

        if header_parts.len() != 4 {
            println!("Invalid file format.");
            std::process::exit(1);
        }

        let _magic_num: String = header_parts[0].clone();
        // let width: u32 = header_parts[1].parse().unwrap();
        // let height: u32 = header_parts[2].parse().unwrap();
        let height: u32 = header_parts[1].parse().unwrap();
        let width: u32 = header_parts[2].parse().unwrap();
        let out_file: String = header_parts[3].clone();

        let mut raytracer = RayTracer::new(width, height);        

        for i in 1..lines.len() {
            let line: &String = &lines[i];
            if line.is_empty() || line.starts_with("#") {
                continue;
            }
    
            let delimitted: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
            let action = delimitted[0].clone();
            let elements: Vec<String> = delimitted[1..delimitted.len()].to_vec();
            
            match action.as_str() {
                "sphere" => {
                    let center = MatVec::new(vec![elements[0].parse().unwrap(),
                                                  elements[1].parse().unwrap(),
                                                  elements[2].parse().unwrap()]);

                    let radius = elements[3].parse().unwrap();
                    let obj = Sphere::new(center, radius, &raytracer.input_state);
                    raytracer.scene.add_shape(Box::new(obj));
                }
                "sun" => {
                    let direction = MatVec::new(vec![elements[0].parse().unwrap(),
                                                  elements[1].parse().unwrap(),
                                                  elements[2].parse().unwrap()]);
                    let obj = Sun::new(direction, &raytracer.input_state);
                    raytracer.scene.add_light_source(Box::new(obj));
                }
                "color" => {
                    let color = MatVec::new(vec![elements[0].parse().unwrap(),
                                                  elements[1].parse().unwrap(),
                                                  elements[2].parse().unwrap()]);
                    raytracer.input_state.color = color;
                }
                "expose" => {
                    let exposure = elements[0].parse().unwrap();
                    raytracer.camera.exposure = Some(exposure);
                }
                "up" => {
                    let up = MatVec::new(vec![elements[0].parse().unwrap(),
                                                  elements[1].parse().unwrap(),
                                                  elements[2].parse().unwrap()]);
                    raytracer.camera.up = up;
                }
                "eye" => {
                    let eye = MatVec::new(vec![elements[0].parse().unwrap(),
                                                  elements[1].parse().unwrap(),
                                                  elements[2].parse().unwrap()]);
                    raytracer.camera.eye = eye;
                }
                "forward" => {
                    let forward = MatVec::new(vec![elements[0].parse().unwrap(),
                                                  elements[1].parse().unwrap(),
                                                  elements[2].parse().unwrap()]);
                    raytracer.camera.forward = forward;
                }
                "fisheye" => {
                    raytracer.camera.projection = ProjectionType::FISHEYE;
                }
                "plane" => {
                    let coeffs = MatVec::<4>::new(vec![elements[0].parse().unwrap(),
                                                  elements[1].parse().unwrap(),
                                                  elements[2].parse().unwrap(),
                                                  elements[3].parse().unwrap()]);
                    let obj = Plane::new(coeffs, &raytracer.input_state);
                    raytracer.scene.add_shape(Box::new(obj));
                }
                _ => {
                    println!("Invalid action: {}", action);
                    std::process::exit(1);
                }
            }

        }    
        raytracer.render();
        raytracer.save_image(out_file);
    }

    pub fn render(&mut self) -> bool {
        for x in 0..self.width {
            for y in 0..self.height {
                
                let ray = Ray::generate_primary_ray(MatVec::new(vec![x as f32, y as f32]), &self.camera);
                // print!("Ray: {:?}", ray);
                // calculate an intersection, from that intersection build out a recursive residual
                let mut pixel_color: RGBA = self.scene.trace_through_scene(&ray, self.bounce_limit);

                // self.image.put_pixel(x, y, image::Rgba([pixel_color.get(0).clone() as u8,
                //                                                pixel_color.get(1).clone() as u8,
                //                                                pixel_color.get(2).clone() as u8,
                //                                                0]));

                if self.camera.exposure.is_some() {
                    pixel_color = utils::appy_exposure(&pixel_color, self.camera.exposure.unwrap());
                }

                #[allow(non_snake_case)]
                let sRGB: RGBA = utils::sRGB(&pixel_color);
                // let sRGB = pixel_color;

                self.image.put_pixel(x, y, sRGB.to_rgba());

                // calculate and intersection point
                // if secondary bounce, execute that action and return a color
                // trace back light sources for the intersection point
                    // for each light source, calculate a ray to the light source
                    // and check the scene for intersections, if intersected, apply shadow and return
                // apply lighting to color
                // sum the colors of all recursive calls and return a final pixel value
                // take that value, apply exposure and sRGB conversit and write to image
            }
        }
        return true;
    }

    pub fn save_image(&self, file_path: String) {
        self.image.save(file_path).unwrap();
    }

}