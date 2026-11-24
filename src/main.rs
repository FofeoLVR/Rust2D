mod math;
mod engine;


use crate::engine::render::shader::Shader;
use gl::types::GLsizeiptr;
use math::transform::Transform;
use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;
use sdl2::video::SwapInterval;
use std::ffi::{CStr, CString};
use std::ptr::null;
use std::time::Instant;
use crate::engine::render::renderer::Renderer;

fn main() {
    let context: Sdl = sdl2::init().unwrap();
    let video_ctx = context.video().unwrap();
    let gl_attr = video_ctx.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_ctx
        .window("Rust2D engine", 720, 720)
        .resizable()
        .opengl()
        .build()
        .unwrap();

    let mut event_pump = context.event_pump().unwrap();
    
    
    unsafe {
        gl::Enable(gl::BLEND);

        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

    }

    let mut renderer:Renderer =  match Renderer::new(){
        Ok(ren) => ren,
        Err(e) => {
            println!("Erreur lors de la creation du renderer: {}", e);
            return;
        }
    };

    let mut transform = Transform::new();
    let mut vsync: bool = true;
    let mut last_frame = Instant::now();
    let mut fps_min: f32 = f32::MAX;
    let mut fps_max: f32 = 0.0;
    let mut fps_timer = Instant::now();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    println!("Quit");
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    println!("Escape");
                    break 'running;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::F),
                    ..
                } => {
                    if vsync {
                        match video_ctx.gl_set_swap_interval(SwapInterval::Immediate) {
                            Ok(()) => {
                                println!("VSync désactivé");
                                vsync = false;
                            }
                            Err(e) => println!("Impossible de désactiver le VSync : {}", e),
                        }
                    } else {
                        match video_ctx.gl_set_swap_interval(SwapInterval::VSync) {
                            Ok(()) => {
                                println!("VSync activé");
                                vsync = true;
                            }
                            Err(e) => println!("Impossible d'activer le VSync : {}", e),
                        }
                    }
                }

                _ => {}
            }
        }

        let frame = Instant::now();
        let delta_time = (frame - last_frame).as_secs_f32();
        last_frame = frame;

        let fps = 1.0 / delta_time;

        fps_min = fps_min.min(fps);
        fps_max = fps_max.max(fps);

        if fps_timer.elapsed().as_secs_f32() >= 1.0 {
            println!("FPS | Min: {:.0} | Max: {:.0}", fps_min, fps_max);

            fps_min = f32::MAX;
            fps_max = 0.0;
            fps_timer = Instant::now();
        }

        unsafe {
            gl::ClearColor(0.0, 0.05, 0.08, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::UseProgram(renderer.program.id());
            gl::BindVertexArray(renderer.vao);
            let rot = transform.rot + 90.0 * delta_time;
            transform.set_rotation(rot);

            let matrix = transform.get_matrix();

            gl::UniformMatrix3fv(renderer.transform_location, 1, gl::TRUE, matrix.data.as_ptr());
            gl::DrawArrays(gl::TRIANGLES, 0, renderer.vertex_count);
        }
        window.gl_swap_window();
    }
}






