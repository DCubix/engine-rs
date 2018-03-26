use std::mem::size_of;
use std::collections::HashMap;
use std::convert::Into;
use std::ptr::null;

use bindings::gl;
use math::vec::*;
use core::util::GLResource;

use shader::*;

struct VertexAttrib {
	pub size: u32,
	pub normalized: bool
}

pub struct VertexFormat {
	attrs: HashMap<String, VertexAttrib>
}

impl VertexFormat {
	pub fn new() -> VertexFormat {
		VertexFormat {
			attrs: HashMap::new()
		}
	}

	pub fn add_attrib(&mut self, name: &str, size: u32, norm: bool) {
		self.attrs.insert(
			name.to_owned(),
			VertexAttrib {
				size,
				normalized: norm
			}
		);
	}

	pub fn size(&self) -> i32 {
		let mut offset = 0;
		for (_, v) in self.attrs.iter() {
			offset += 4 * v.size;
		}
		offset as i32
	}

	pub fn bind_attribs(&self, shader: &mut Shader) {
		let stride = self.size();
		let mut offset = 0u32;
		for (k, v) in self.attrs.iter() {
			let loc = shader.get_attrib_location(&k);
			if loc != -1 {
				unsafe {
					gl::EnableVertexAttribArray(loc as u32);
					gl::VertexAttribPointer(
						loc as u32,
						v.size as i32,
						gl::FLOAT,
						if v.normalized { gl::TRUE } else { gl::FALSE },
						stride,
						offset as *const _
					);
				}
			}
			offset += 4 * v.size;
		}
	}

	pub fn unbind_attribs(&self, shader: &mut Shader) {
		for (k, _) in self.attrs.iter() {
			let loc = shader.get_attrib_location(&k);
			if loc != -1 {
				unsafe {
					gl::DisableVertexAttribArray(loc as u32);
				}
			}
		}
	}
}

pub trait Vertex: Copy + Clone {
	fn get_format(&self) -> VertexFormat;
}

pub struct Mesh<V: Vertex> {
	vbo: u32,
	ibo: u32,
	format: VertexFormat,
	indexed: bool,
	vertices: Vec<V>,
	indices: Vec<u16>,
	vbo_size: u32,
	ibo_size: u32
}

fn create_buffer() -> u32 {
	unsafe {
		let mut buff = 0u32;
		gl::GenBuffers(1, &mut buff);
		buff
	}
}

impl<V> Mesh<V> where V: Vertex {
	pub fn new(indexed: bool) -> Mesh<V> {
		Mesh {
			vbo: create_buffer(),
			ibo: if indexed { create_buffer() } else { 0 },
			format: VertexFormat::new(),
			indexed,
			vertices: Vec::new(),
			indices: Vec::new(),
			vbo_size: 0,
			ibo_size: 0
		}
	}

	pub fn clear(&mut self) {
		self.vertices.clear();
		self.indices.clear();
	}

	pub fn vertex_count(&self) -> usize { self.vertices.len() }
	pub fn index_count(&self) -> usize { self.indices.len() }

	pub fn flush(&mut self) {
		unsafe {
			let vsize = self.format.size() as u32 * self.vertices.len() as u32;

			gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
			if vsize > self.vbo_size {
				gl::BufferData(gl::ARRAY_BUFFER, vsize as _, self.vertices.as_ptr() as _, gl::DYNAMIC_DRAW);
				self.vbo_size = vsize;
			} else {
				gl::BufferSubData(gl::ARRAY_BUFFER, 0, vsize as _, self.vertices.as_ptr() as _);
			}

			if self.indexed {
				let esize = size_of::<u16>() as u32 * self.indices.len() as u32;

				gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
				if esize > self.ibo_size {
					gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, esize as _, self.indices.as_ptr() as _, gl::DYNAMIC_DRAW);
					self.ibo_size = esize;
				} else {
					gl::BufferSubData(gl::ELEMENT_ARRAY_BUFFER, 0, esize as _, self.indices.as_ptr() as _);
				}
			}
		}
	}

	pub fn add_vertex(&mut self, v: V) {
		self.vertices.push(v);
	}

	pub fn add_index(&mut self, i: u16) {
		self.indices.push(i);
	}

	pub fn add_triangle(&mut self, i0: u16, i1: u16, i2: u16) {
		self.indices.push(i0);
		self.indices.push(i1);
		self.indices.push(i2);
	}

	pub fn render(&self, mode: u32, shader: &mut Shader) {
		unsafe {
			if self.indexed {
				gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo);
			}
			gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
		}
		self.format.bind_attribs(shader);

		unsafe {
			if self.indexed {
				gl::DrawElements(mode, self.index_count() as _, gl::UNSIGNED_SHORT, null());
			} else {
				gl::DrawArrays(mode, 0, self.vertex_count() as _);
			}
		}

		self.format.unbind_attribs(shader);
		unsafe {
			gl::BindBuffer(gl::ARRAY_BUFFER, 0);
			if self.indexed {
				gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
			}
		}
	}
}
