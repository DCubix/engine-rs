// Original code from <https://github.com/manpat/common-rs>
use std::ops::{ Add, Sub, Mul, Div, Neg, Index, IndexMut };
use std::ops::{ AddAssign, SubAssign, MulAssign, DivAssign };

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vec2 { pub x: f32, pub y: f32 }

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2i { pub x: i32, pub y: i32 }

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vec3 { pub x: f32, pub y: f32, pub z: f32 }

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vec4 { pub x: f32, pub y: f32, pub z: f32, pub w: f32 }

impl Vec2 {
	pub fn new(x: f32, y: f32) -> Vec2 { Vec2 { x, y } }
	pub fn uniform(v: f32) -> Vec2 { Vec2::new(v, v) }
	pub fn zero() -> Vec2 { Vec2::uniform(0.0) }
	pub fn from_angle(a: f32) -> Vec2 { Vec2::new(a.cos(), a.sin()) }

	pub fn as_tuple(self) -> (f32, f32) { (self.x, self.y) }
	pub fn as_vec2i(self) -> Vec2i { Vec2i::new(self.x as i32, self.y as i32) }

	pub fn extend(self, z: f32) -> Vec3 { Vec3::new(self.x, self.y, z) }

	pub fn angle(self) -> f32 { self.y.atan2(self.x) }
	pub fn dot(self, o: Vec2) -> f32 { self.x * o.x + self.y * o.y }
	pub fn perp_dot(self, o: Vec2) -> f32 { self.x * o.y - self.y * o.x }
	pub fn perp(self) -> Vec2 { Vec2::new(-self.y, self.x) }
	pub fn len(self) -> f32 { self.dot(self).sqrt() }
	pub fn normalized(self) -> Vec2 { self * (1.0 / self.len()) }
}

impl Vec3 {
	pub fn new(x: f32, y: f32, z: f32) -> Vec3 { Vec3 { x, y, z } }
	pub fn uniform(v: f32) -> Vec3 { Vec3::new(v, v, v) }
	pub fn zero() -> Vec3 { Vec3::uniform(0.0) }

	pub fn as_tuple(self) -> (f32, f32, f32) { (self.x, self.y, self.z) }
	pub fn as_vec2(self) -> Vec2 { Vec2::new(self.x, self.y) }

	pub fn extend(self, w: f32) -> Vec4 { Vec4::new(self.x, self.y, self.z, w) }

	pub fn dot(self, o: Vec3) -> f32 { self.x * o.x + self.y * o.y + self.z * o.z }
	
	pub fn cross(self, o: Vec3) -> Vec3 {
		Vec3::new(
			self.y * o.z - self.z * o.y,
			self.z * o.x - self.x * o.z,
			self.x * o.y - self.y * o.x
		)
	}

	pub fn len(self) -> f32 { self.dot(self).sqrt() }

	pub fn normalized(self) -> Vec3 { self * (1.0 / self.len()) }
}

impl Vec4 {
	pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 { Vec4 { x, y, z, w } }
	pub fn uniform(v: f32) -> Vec4 { Vec4::new(v, v, v, v) }
	pub fn zero() -> Vec4 { Vec4::uniform(0.0) }

	pub fn from_slice(s: &[f32]) -> Vec4 {
		assert!(s.len() >= 4);
		Vec4::new(s[0], s[1], s[2], s[3])
	}

	pub fn as_tuple(self) -> (f32, f32, f32, f32) { (self.x, self.y, self.z, self.w) }
	pub fn as_vec3(self) -> Vec3 { Vec3::new(self.x, self.y, self.z) }

	pub fn dot(self, o: Vec4) -> f32 { self.x * o.x + self.y * o.y + self.z * o.z + self.w * o.w }
	
	pub fn len(self) -> f32 { self.dot(self).sqrt() }

	pub fn normalized(self) -> Vec4 { self * (1.0 / self.len()) }
}

impl Index<usize> for Vec4 {
	type Output = f32;
	fn index(&self, i: usize) -> &f32 {
		match i {
			0 => { &self.x },
			1 => { &self.y },
			2 => { &self.z },
			_ => { &self.w }
		}
	}
}

impl IndexMut<usize> for Vec4 {
	fn index_mut(&mut self, i: usize) -> &mut f32 {
		match i {
			0 => { &mut self.x },
			1 => { &mut self.y },
			2 => { &mut self.z },
			_ => { &mut self.w }
		}
	}
}

impl Vec2i {
	pub fn new(x: i32, y: i32) -> Vec2i { Vec2i { x, y } }
	pub fn uniform(v: i32) -> Vec2i { Vec2i::new(v, v) }
	pub fn zero() -> Vec2i { Vec2i::uniform(0) }
	pub fn from_tuple(t: (i32, i32)) -> Vec2i { Vec2i::new(t.0, t.1) }

	pub fn as_tuple(self) -> (i32, i32) { (self.x, self.y) }
	pub fn as_vec2(self) -> Vec2 { Vec2::new(self.x as f32, self.y as f32) }

	pub fn len(self) -> f32 { ((self.x * self.x + self.y * self.y) as f32).sqrt() }
}

macro_rules! impl_vector_bin_op {
	($ty:ident, $trait:ident<$scalar:ty>, $fn:ident, $op:tt, $($els:ident),+) => {
		impl $trait for $ty {
			type Output = $ty;
			fn $fn(self, o: $ty) -> $ty {
				$ty::new($(self.$els $op o.$els),+)
			}
		}

		impl $trait<$scalar> for $ty {
			type Output = $ty;
			fn $fn(self, o: $scalar) -> $ty {
				$ty::new($(self.$els $op o),+)
			}
		}
	};

	(ass $ty:ident, $trait:ident<$scalar:ty>, $fn:ident, $op:tt, $($els:ident),+) => {
		impl $trait for $ty {
			fn $fn(&mut self, o: $ty) {
				$(
					self.$els $op o.$els;
				)+
			}
		}

		impl $trait<$scalar> for $ty {
			fn $fn(&mut self, o: $scalar) {
				$(
					self.$els $op o;
				)+
			}
		}
	};
}

macro_rules! bulk_impl_vector_ops {
	($ty:ident, $scalar:ty, $($els:ident),+) => {
		impl_vector_bin_op!($ty, Add<$scalar>, add, +, $($els),+);
		impl_vector_bin_op!($ty, Sub<$scalar>, sub, -, $($els),+);
		impl_vector_bin_op!($ty, Mul<$scalar>, mul, *, $($els),+);
		impl_vector_bin_op!($ty, Div<$scalar>, div, /, $($els),+);

		impl_vector_bin_op!(ass $ty, AddAssign<$scalar>, add_assign, +=, $($els),+);
		impl_vector_bin_op!(ass $ty, SubAssign<$scalar>, sub_assign, -=, $($els),+);
		impl_vector_bin_op!(ass $ty, MulAssign<$scalar>, mul_assign, *=, $($els),+);
		impl_vector_bin_op!(ass $ty, DivAssign<$scalar>, div_assign, /=, $($els),+);

		impl Neg for $ty {
			type Output = $ty;
			fn neg(self) -> $ty {
				$ty::new($(-self.$els),+)
			}
		}
	};
}

bulk_impl_vector_ops!(Vec2, f32, x, y);
bulk_impl_vector_ops!(Vec3, f32, x, y, z);
bulk_impl_vector_ops!(Vec4, f32, x, y, z, w);
bulk_impl_vector_ops!(Vec2i, i32, x, y);