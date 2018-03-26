use bindings::emscripten::*;
use bindings::gl;

pub struct Context { ctx: EMSCRIPTEN_WEBGL_CONTEXT_HANDLE }

impl Context {
	pub fn new(depth: bool, alpha: bool) -> Context {
		use std::mem::uninitialized;

		let ems_context_handle = unsafe {
			let mut attribs = uninitialized();
			emscripten_webgl_init_context_attributes(&mut attribs);

			attribs.alpha = if alpha { 1 } else { 0 };
			attribs.stencil = 1;
			attribs.antialias = 1;
			attribs.preserveDrawingBuffer = 0;
			attribs.enableExtensionsByDefault = 1;
			attribs.depth = if depth { 1 } else { 0 };

			emscripten_webgl_create_context(b"canvas\0".as_ptr() as _, &attribs)
		};

		match ems_context_handle {
			EMSCRIPTEN_RESULT_NOT_SUPPORTED => {
				panic!("WebGL not supported");
			}

			EMSCRIPTEN_RESULT_FAILED_NOT_DEFERRED => {
				panic!("WebGL context creation failed (FAILED_NOT_DEFERRED)");
			}

			EMSCRIPTEN_RESULT_FAILED => {
				panic!("WebGL context creation failed (FAILED)");
			}

			x if x < 0 => {
				panic!("WebGL context creation failed ({})", x);
			}

			_ => {}
		}

		if unsafe { emscripten_webgl_make_context_current(ems_context_handle) != EMSCRIPTEN_RESULT_SUCCESS } {
			panic!("Failed to make WebGL context current.");
		}

		Context { ctx: ems_context_handle }
	}

	pub fn clear(&self, flags: u32) {
		unsafe {
			gl::Clear(flags);
		}
	}

	pub fn set_clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
		unsafe {
			gl::ClearColor(r, g, b, a);
		}
	}

	pub fn set_viewport(&self, x: i32, y: i32, w: i32, h: i32) {
		unsafe {
			gl::Viewport(x, y, w, h);
		}
	}

	pub fn destroy(&self) {
		unsafe {
			emscripten_webgl_destroy_context(self.ctx);
		}
	}
}