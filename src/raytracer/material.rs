pub struct Material {
    pub shininess: Vec<f32>,
}


impl Material {

    pub fn new(shininess: Vec<f32>) -> Material {
        Material {
            shininess,
        }
    }

}