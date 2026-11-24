use std::time::Instant;
use crate::math::transform::Transform;
use sdl2::{
    Sdl,
    VideoSubsystem,
    EventPump,
    video::{Window, GLContext},
};
use crate::engine::render::renderer::Renderer;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::SwapInterval;

pub struct Engine{
    renderer: Renderer,
    gl_context: GLContext,
    window: Window,
    event_pump: EventPump,
    video_subsystem: VideoSubsystem,
    sdl: Sdl,

    last_frame: Instant,
    transform: Transform,
}

impl Engine{
    pub(crate) fn new(window_title:&str, window_width:u32, window_height:u32) -> Result<Engine, String>{
        let sdl:Sdl = sdl2::init()?;
        let video_subsystem = sdl.video()?;
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);
        let window = video_subsystem
            .window(window_title, window_width, window_height)
            .resizable()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;
        let gl_context = window.gl_create_context()?;
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);
        let event_pump = sdl.event_pump()?;
        let renderer = Renderer::new()?;
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        Ok(Engine{
            renderer,
            gl_context,
            window,
            event_pump,
            video_subsystem,
            sdl,
            last_frame: Instant::now(),
            transform: Transform::new()
        })
    }

    pub(crate) fn run(&mut self){
        let mut running = true;
        let mut vsync: bool = true;
        
        

        while running{
            for event in self.event_pump.poll_iter(){
                match event {
                    Event::Quit {..} => {
                        running = false;
                        break;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::F),
                        repeat: false,
                        ..
                    } => {
                        if vsync {
                            match self.video_subsystem.gl_set_swap_interval(SwapInterval::Immediate) {
                                Ok(()) => {
                                    println!("VSync désactivé");
                                    vsync = false;
                                }
                                Err(e) => println!("Impossible de désactiver le VSync : {}", e),
                            }
                        } else {
                            match self.video_subsystem.gl_set_swap_interval(SwapInterval::VSync) {
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

            if !running {
                break;
            }
            
            let current_frame = Instant::now();
            let delta_time = (current_frame - self.last_frame).as_secs_f32();
            self.last_frame = current_frame;
            self.update(delta_time);
            self.render();
        }
    }
    
    fn update(&mut self, delta_time:f32){
        let rot = self.transform.rot + 90.0 * delta_time;
        self.transform.set_rotation(rot);
    }
    
    fn render(&mut self){
        self.renderer.render(&mut self.transform);
        self.window.gl_swap_window();
    }
}