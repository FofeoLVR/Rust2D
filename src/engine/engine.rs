use sdl2::{
    Sdl,
    VideoSubsystem,
    EventPump,
    video::{Window, GLContext},
};
use crate::engine::render::renderer::Renderer;

pub struct Engine{
    renderer: Renderer,
    gl_context: GLContext,
    window: Window,
    event_pump: EventPump,
    video_subsystem: VideoSubsystem,
    sdl: Sdl,
}

impl Engine{
    pub(crate) fn new(window_title:&str, window_width:u32, window_height:u32) -> Result<Engine, String>{
        let sdl:Sdl = sdl2::init()?;
        let video_subsystem = sdl.video()?;
        
        let window = video
            .window(window_title, window_width, window_height)
            .resizable()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;
        let gl_context = window.gl_create_context()?;
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);
        let event_pump = sdl.event_pump()?;
        let renderer = Renderer::new()?;
           
        
        Ok(Engine{
            renderer,
            gl_context,
            window,
            event_pump,
            video_subsystem,
            sdl
        })
    }
}