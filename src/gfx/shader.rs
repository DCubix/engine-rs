use bindings::gl;
use math::vec::*;
use math::mat::*;
use core::util::GLResource;

use std::collections::HashMap;
use std::ffi::{ CString, CStr };
use std::ptr;

pub trait Setter<T> {
	fn set(&self, val: T);
}

pub struct Uniform { pub loc: i32 }

impl Setter<i32> for Uniform {
	fn set(&self, val: i32) {
		unsafe {
			gl::Uniform1i(self.loc, val);
		}
	}
}

impl Setter<f32> for Uniform {
	fn set(&self, val: f32) {
		unsafe {
			gl::Uniform1f(self.loc, val);
		}
	}
}

impl Setter<Vec2> for Uniform {
	fn set(&self, val: Vec2) {
		unsafe {
			gl::Uniform2f(self.loc, val.x, val.y);
		}
	}
}

impl Setter<Vec3> for Uniform {
	fn set(&self, val: Vec3) {
		unsafe {
			gl::Uniform3f(self.loc, val.x, val.y, val.z);
		}
	}
}

impl Setter<Vec4> for Uniform {
	fn set(&self, val: Vec4) {
		unsafe {
			gl::Uniform4f(self.loc, val.x, val.y, val.z, val.w);
		}
	}
}

impl Setter<Mat4> for Uniform {
	fn set(&self, val: Mat4) {
		unsafe {
			gl::UniformMatrix4fv(self.loc, 1, gl::FALSE, val.as_ptr());
		}
	}
}

pub struct Shader {
	program: u32,
	uniforms: HashMap<String, i32>,
	attribs: HashMap<String, i32>
}

impl Shader {
	pub fn new(vert: &str, frag: &str) -> Shader {
		let prog;
		unsafe {
			prog = gl::CreateProgram();

			let vs = match Shader::create_shader(vert, gl::VERTEX_SHADER) {
				Some(i) => i,
				None => panic!("Invalid Vertex Shader.")
			};

			let fs = match Shader::create_shader(frag, gl::FRAGMENT_SHADER) {
				Some(i) => i,
				None => panic!("Invalid Fragment Shader.")
			};

			gl::AttachShader(prog, vs);
			gl::AttachShader(prog, fs);
			gl::LinkProgram(prog);

			gl::DeleteShader(vs);
			gl::DeleteShader(fs);
		}

		Shader {
			program: prog,
			uniforms: HashMap::new(),
			attribs: HashMap::new()
		}
	}

	pub fn get_uniform_location(&mut self, name: &str) -> i32 {
		if !self.uniforms.contains_key(name) {
			let cstr = CString::new(name).unwrap();
			let loc;
			unsafe {
				loc = gl::GetUniformLocation(self.program, cstr.as_ptr());
			}
			if loc > -1 {
				self.uniforms.insert(name.to_owned(), loc);
			}
		}
		match self.uniforms.get(name) {
			Some(loc) => { *loc },
			None => { -1 }
		}
	}

	pub fn get_attrib_location(&mut self, name: &str) -> i32 {
		if !self.attribs.contains_key(name) {
			let cstr = CString::new(name).unwrap();
			let loc;
			unsafe {
				loc = gl::GetAttribLocation(self.program, cstr.as_ptr());
			}
			if loc > -1 {
				self.attribs.insert(name.to_owned(), loc);
			}
		}
		match self.attribs.get(name) {
			Some(loc) => { *loc },
			None => { -1 }
		}
	}

	pub fn get(&mut self, uniform_name: &str) -> Option<Uniform> {
		let loc = self.get_uniform_location(uniform_name);
		if loc == -1 {
			return None;
		}
		Some(Uniform { loc })
	}

	pub fn bind(&self) {
		unsafe { gl::UseProgram(self.program); }
	}

	pub fn unbind(&self) {
		unsafe { gl::UseProgram(0); }
	}

	fn create_shader(src: &str, ty: gl::GLenum) -> Option<u32> {
		unsafe {
			let shader = gl::CreateShader(ty);

			let c_str = CString::new(src).unwrap();
			gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
			gl::CompileShader(shader);

			let mut status = 0i32;
			gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
			if status == 0 {
				let mut buf = [0u8; 1024];
				let mut len = 0i32;
				gl::GetShaderInfoLog(shader, buf.len() as i32, &mut len, buf.as_mut_ptr() as *mut _);

				println!("{}", CStr::from_bytes_with_nul_unchecked(&buf[..len as usize]).to_str().unwrap());
				return None;
			}

			Some(shader)
		}
	}

}

impl GLResource for Shader {
	fn destroy(&self) {
		unsafe {
			gl::DeleteProgram(self.program);
		}
	}
}