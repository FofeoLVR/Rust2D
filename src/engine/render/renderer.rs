use std::ffi::CString;
use gl::types::GLsizeiptr;
use gl::types::GLsizei;
use super::shader::Program;


const VERTEX_SIZE: usize = 7;
pub struct Renderer {
    pub program: Program,
    pub vao: u32,
    pub vbo: u32,
    pub vertex_count: GLsizei,
    pub transform_location: i32,
}

impl Drop for Renderer{
    fn drop(&mut self){
        unsafe {
            gl::DeleteVertexArrays(1,&mut self.vao);
            gl::DeleteBuffers(1, &mut self.vbo);
        }

    }
}

impl Renderer{
    pub (crate) fn new() -> Result<Renderer, String>{
        let program:Program = super::shader::create_shader_program()?;
        let transform_name = CString::new("uTransform").unwrap();
        let transform_location;
        let mut vbo = 0;
        let mut vao = 0;
        let vertices = get_vertices();
        let vertex_count = (vertices.len() / VERTEX_SIZE) as GLsizei;
        unsafe {
            transform_location = gl::GetUniformLocation(program.id(), transform_name.as_ptr());
            if transform_location == -1 {
                return Err("Uniform uTransform introuvable".to_string());
            }

            gl::UseProgram(program.id());
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(&vertices) as GLsizeiptr,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                28,
                0 as *const _,
            );

            gl::VertexAttribPointer(
                1,
                4,
                gl::FLOAT,
                gl::FALSE,
                28,
                12 as *const _,
            );



            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);

        }
        
        Ok(Renderer{
            
            program,
            vao,
            vbo,
            vertex_count,
            transform_location
        })


            
    }
}


fn get_vertices() -> [f32; 21]{
    [
        -0.5, -0.289, 0.0, 1.0, 0.0, 0.0, 0.5, 0.5, -0.289, 0.0, 0.0, 1.0, 0.0, 0.5, 0.0, 0.577,
        0.0, 0.0, 0.0, 1.0, 0.5,
    ]

}