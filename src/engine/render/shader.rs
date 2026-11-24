use std::ffi::{CStr, CString};


pub struct Shader {
    id: u32,
}

impl  Shader{
    pub(crate) fn id(&self) -> u32 {
        self.id
    }
}
impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            if self.id != 0{
                gl::DeleteShader(self.id);
            }

        }
    }
}

pub struct Program {
    id: u32,
}

impl Program {
    pub(crate) fn id(&self) -> u32 {
        self.id
    }
}

impl Drop for Program{
    fn drop(&mut self) {
        unsafe {
            if self.id != 0 {
                gl::DeleteProgram(self.id)
            }

        }
    }
}



pub (crate) fn get_shader(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

pub (crate) fn compile_shader(source: &str, shader_type: u32) -> Result<Shader, String> {
    let c_source = CString::new(source)
        .map_err(|e| e.to_string())?;

    unsafe {
        let shader_id = gl::CreateShader(shader_type);

        let source_ptr = c_source.as_ptr();

        gl::ShaderSource(shader_id, 1, &source_ptr, std::ptr::null());

        gl::CompileShader(shader_id);

        let mut success = gl::FALSE as gl::types::GLint;

        gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);

        let mut info_log = vec![0i8; 512];

        if success == gl::FALSE as gl::types::GLint {
            println!("Échec de compilation du shader !");
            gl::GetShaderInfoLog(shader_id, 512, std::ptr::null_mut(), info_log.as_mut_ptr());
        } else {
            println!("Shader compilé avec succès !");
        }

        let error_message = CStr::from_ptr(info_log.as_ptr());

        if success == gl::FALSE as gl::types::GLint {
            gl::DeleteShader(shader_id);
            Err(error_message.to_string_lossy().into_owned())
        } else {
            Ok(Shader {
                id: shader_id,
            })
        }
    }
}

pub (crate)fn create_shader_program() -> Result<Program, String> {
    let vert_shader_txt = get_shader("src/engine/render/shaders/basic.vert").map_err(|e| e.to_string())?;
    let vert_shader:Shader = compile_shader(&vert_shader_txt, gl::VERTEX_SHADER)?;
    let frag_shader_txt = get_shader("src/engine/render/shaders/basic.frag").map_err(|e| e.to_string())?;
    let frag_shader:Shader = compile_shader(&frag_shader_txt, gl::FRAGMENT_SHADER)?;

    unsafe {
        let mut shader_program:Program = Program { id:0 };
        shader_program.id = gl::CreateProgram();
        if shader_program.id() == 0 {
            return Err("Impossible de créer le programme OpenGL".to_string());
        }

        gl::AttachShader(shader_program.id(), vert_shader.id());
        gl::AttachShader(shader_program.id(), frag_shader.id());
        gl::LinkProgram(shader_program.id());


        let mut success = gl::FALSE as gl::types::GLint;

        gl::GetProgramiv(shader_program.id(), gl::LINK_STATUS, &mut success);

        let mut info_log = vec![0i8; 512];

        if success == gl::FALSE as gl::types::GLint {
            println!("Program failed");
            gl::GetProgramInfoLog(
                shader_program.id(),
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr(),
            );

            let error_message = CStr::from_ptr(info_log.as_ptr())
                .to_string_lossy()
                .into_owned();

            drop(shader_program);

            Err(error_message)
        } else {
            println!("Program success");
            Ok(shader_program)
        }
    }
}