#![feature(specialization)]
#![feature(ord_max_min)]
#![feature(ascii_ctype)]
#![feature(link_args)]
#![feature(const_fn)]
#![feature(splice)]

#[macro_use] pub mod bindings;
pub mod math;
pub mod core;
pub mod gfx;

pub use bindings::emscripten::*;
pub use math::*;
pub use core::*;
pub use gfx::*;