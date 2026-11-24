use crate::engine::engine::Engine;
use sdl2;

mod math;
mod engine;

fn main() {
    let mut engine = match Engine::new("Rust 2D", 720, 720){
        Ok(e) =>  e,
        Err(e) => {
            println!("Error while creating engine : {}",e  );
            std::process::exit(0)
        }
    };

    engine.run();
}






