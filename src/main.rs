
extern crate new_rt;

use new_rt::get_info_map;
use new_rt::{Canvas, Renderer};

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Error: wrong number of argument, expeted 1 get {}", args.len() - 1);
        std::process::exit(1);
    }

    let map_name = String::from(args[1].clone());
    match get_info_map(&map_name) {
        Ok (info_map) => {
            eprintln!("{:?}", info_map);
            let canvas = Canvas::new(info_map.canvas);
            match Renderer::new(canvas, info_map.world) {
                Ok(mut renderer) => {
                    renderer.update_image();
                    let _ = renderer.render();
                }
                Err(e) => {
                    eprintln!("Error from renderer creation: {:?}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error from get_info_map: {}", e);
            std::process::exit(1);
        }
    }
    ()
}
