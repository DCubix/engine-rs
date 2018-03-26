// Original code from <https://github.com/manpat/common-rs>
use std::ops::{ Add, Mul };
use mat::*;
use vec::*;

#[derive(Copy, Clone, Debug)]
pub struct Quat{ pub x: f32, pub y: f32, pub z: f32, pub w: f32 }

impl Quat {
	pub const fn from_raw(x: f32, y: f32, z: f32, w: f32) -> Quat {
		Quat { x, y, z, w }
	}

	pub fn new(axis: Vec3, angle: f32) -> Quat {
		let angle = angle / 2.0;
		let s = angle.sin();

		Quat::from_raw(
			axis.x * s,
			axis.y * s,
			axis.z * s,
			angle.cos()
		)
	}

	pub fn forward(&self) -> Vec3 { *self * Vec3::from_z(-1.0) }
	pub fn right(&self) -> Vec3 { *self * Vec3::from_x(1.0) }
	pub fn up(&self) -> Vec3 { *self * Vec3::from_y(1.0) }

	pub fn imaginary(&self) -> Vec3 {
		Vec3::new(self.x, self.y, self.z)
	}

	pub fn magnitude(&self) -> f32 {
		(self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
	}

	pub fn normalize(&self) -> Quat {
		let m = self.magnitude();
		Quat::from_raw(self.x/m, self.y/m, self.z/m, self.w/m)
	}

	pub fn conjugate(&self) -> Quat {
		Quat::from_raw(-self.x, -self.y, -self.z, self.w)
	}

	pub fn to_mat4(&self) -> Mat4 {
		Mat4::from_rows([
			(*self * Vec3::new(1.0, 0.0, 0.0)).extend(0.0),
			(*self * Vec3::new(0.0, 1.0, 0.0)).extend(0.0),
			(*self * Vec3::new(0.0, 0.0, 1.0)).extend(0.0),
			Vec4::new(0.0, 0.0, 0.0, 1.0)
		]).transpose()
	}
}

impl Add<Quat> for Quat {
	type Output = Quat;
	fn add(self, o: Quat) -> Quat {
		Quat::from_raw(self.x+o.x, self.y+o.y, self.z+o.z, self.w+o.w)
	}
}

impl Mul<Quat> for Quat {
	type Output = Quat;
	fn mul(self, o: Quat) -> Quat {
		Quat::from_raw(
			 self.w*o.x - self.z*o.y + self.y*o.z + self.x*o.w,
			 self.z*o.x + self.w*o.y - self.x*o.z + self.y*o.w,
			-self.y*o.x + self.x*o.y + self.w*o.z + self.z*o.w,
			-self.x*o.x - self.y*o.y - self.z*o.z + self.w*o.w
		)
	}
}

impl Mul<f32> for Quat {
	type Output = Quat;
	fn mul(self, o: f32) -> Quat {
		Quat::from_raw(self.x*o, self.y*o, self.z*o, self.w*o)
	}
}

impl Mul<Vec3> for Quat {
	type Output = Vec3;
	fn mul(self, o: Vec3) -> Vec3 {
		let q = Quat::from_raw(o.x, o.y, o.z, 0.0);
		(self * q * self.conjugate()).imaginary()
	}
}