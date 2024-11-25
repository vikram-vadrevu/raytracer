mod raytracer;
use raytracer::raytracer::RayTracer;
fn main() {
    
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }
    let input_file = &args[1];
    RayTracer::render_from_file(input_file);
}
