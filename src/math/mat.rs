use std::ops::{ Mul, Index, IndexMut };
use vec::*;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Mat4 { pub rows: [Vec4; 4] }

impl Index<usize> for Mat4 {
	type Output = Vec4;
	fn index(&self, i: usize) -> &Vec4 {
		&self.rows[i]
	}
}

impl IndexMut<usize> for Mat4 {
	fn index_mut(&mut self, i: usize) -> &mut Vec4 {
		&mut self.rows[i]
	}
}

impl Mat4 {
	pub fn as_ptr(&self) -> *const f32 {
		&self.rows[0][0]
	}

	pub fn new(m: &[f32; 16]) -> Mat4 {
		Mat4 {
			rows: [
				Vec4::from_slice(&m[0..4]),
				Vec4::from_slice(&m[4..8]),
				Vec4::from_slice(&m[8..12]),
				Vec4::from_slice(&m[12..16]),
			]
		}
	}

	pub fn from_rows(r0: Vec4, r1: Vec4, r2: Vec4, r3: Vec4) -> Mat4 {
		Mat4 { rows: [ r0, r1, r2, r3 ] }
	}

	pub fn ident() -> Mat4 {
		Mat4::new(&[
			1.0, 0.0, 0.0, 0.0,
			0.0, 1.0, 0.0, 0.0,
			0.0, 0.0, 1.0, 0.0,
			0.0, 0.0, 0.0, 1.0
		])
	}

	pub fn translation(v: Vec3) -> Mat4 {
		Mat4::new(&[
			1.0, 0.0, 0.0, 0.0,
			0.0, 1.0, 0.0, 0.0,
			0.0, 0.0, 1.0, 0.0,
			v.x, v.y, v.z, 1.0
		])
	}

	pub fn uniform_scaling(v: f32) -> Mat4 {
		Mat4::new(&[
			  v, 0.0, 0.0, 0.0,
			0.0,   v, 0.0, 0.0,
			0.0, 0.0,   v, 0.0,
			0.0, 0.0, 0.0, 1.0
		])
	}

	pub fn scaling(v: Vec3) -> Mat4 {
		Mat4::new(&[
			v.x, 0.0, 0.0, 0.0,
			0.0, v.y, 0.0, 0.0,
			0.0, 0.0, v.z, 0.0,
			0.0, 0.0, 0.0, 1.0
		])
	}

	pub fn rotation_x(a: f32) -> Mat4 {
		let (s, c) = a.sin_cos();
		Mat4::new(&[
			1.0, 0.0, 0.0, 0.0,
			0.0,   c,  -s, 0.0,
			0.0,   s,   c, 0.0,
			0.0, 0.0, 0.0, 1.0
		])
	}

	pub fn rotation_y(a: f32) -> Mat4 {
		let (s, c) = a.sin_cos();
		Mat4::new(&[
			  c, 0.0,  -s, 0.0,
			0.0, 1.0, 0.0, 0.0,
			  s, 0.0,   c, 0.0,
			0.0, 0.0, 0.0, 1.0
		])
	}

	pub fn rotation_z(a: f32) -> Mat4 {
		let (s, c) = a.sin_cos();
		Mat4::new(&[
			  c,  -s, 0.0, 0.0,
			  s,   c, 0.0, 0.0,
			0.0, 0.0, 1.0, 0.0,
			0.0, 0.0, 0.0, 1.0
		])
	}

	pub fn axis_angle(axis: Vec3, a: f32) -> Mat4 {
		let (s, c) = a.sin_cos();
		let t = 1.0 - c;
		let ax = axis.normalized();
		let x = ax.x;
		let y = ax.y;
		let z = ax.z;
		Mat4::new(&[
			t * x * x + c, t * x * y - z * s, t * x * z + y * s, 0.0,
			t * x * y + z * s, t * y * y + c, t * y * z - x * s, 0.0,
			t * x * z - y * s, t * y * z + x * s, t * z * z + c, 0.0,
			0.0, 0.0, 0.0, 1.0
		])
	}

	pub fn ortho(l: f32, r: f32, t: f32, b: f32, n: f32, f: f32) -> Mat4 {
		let w = r - l;
		let h = t - b;
		let d = f - n;
		Mat4::new(&[
			2.0 / w,	 0.0,	   0.0, -(r + l) / w,
				0.0, 2.0 / h,	   0.0, -(t + b) / h,
				0.0,	 0.0, -2.0 / d, -(f + n) / d,
				0.0,	 0.0,	   0.0,			 1.0,
		])
	}

	pub fn ortho_2d(width: f32, height: f32) -> Mat4 {
		Mat4::ortho(0.0, width, height, 0.0, -1.0, 1.0)
	}

	pub fn frustum(l: f32, r: f32, t: f32, b: f32, n: f32, f: f32) -> Mat4 {
		let n2 = 2.0 * n;
		let w = r - l;
		let h = t - b;
		let d = f - n;

		Mat4::new(&[
			n2 / w, 0.0, 0.0, 0.0,
			0.0, n2 / h, 0.0, 0.0,
			(r + l) / w, (t + b) / h, (-f - n) / d, -1.0,
			0.0, 0.0, (-n2 * f) / d, 0.0
		])
	}

	pub fn perspective(fov: f32, asp: f32, n: f32, f: f32) -> Mat4 {
		let ymax = n * fov.tan();
		let xmax = ymax * asp;
		Mat4::frustum(-xmax, xmax, -ymax, ymax, n, f)
	}

	pub fn look_at(eye: Vec3, at: Vec3, up: Vec3) -> Mat4 {
		let z = (eye - at).normalized();
		let x = up.cross(z).normalized();
		let y = z.cross(x);

		let r = Mat4::new(&[
			x.x, x.y, -x.z, 0.0,
			y.x, y.y, -y.z, 0.0,
			z.x, z.y, -z.z, 0.0,
			0.0, 0.0, 0.0, 1.0
		]);

		Mat4::translation(-eye) * r
	}

	pub fn transposed(self) -> Mat4 {
		let [a, b, c, d] = self.rows;
		Mat4::new(&[
			a.x, b.x, c.x, d.x,
			a.y, b.y, c.y, d.y,
			a.z, b.z, c.z, d.z,
			a.w, b.w, c.w, d.w
		])
	}

	pub fn inverted(self) -> Mat4 {
		//
		// Inversion by Cramer's rule.  Code taken from an Intel publication
		//
		let mut mat = self.clone();
		let mut tmp = [0.0f32; 12];
		let mut src = [0.0f32; 16];

		// Transpose
		for i in 0..4 {
			src[i + 0] = self[i][0];
			src[i + 4] = self[i][1];
			src[i + 8] = self[i][2];
			src[i + 12] = self[i][3];
		}

		// Calculate pairs for first 8 elements (cofactors)
		tmp[0] = src[10] * src[15];
		tmp[1] = src[11] * src[14];
		tmp[2] = src[9] * src[15];
		tmp[3] = src[11] * src[13];
		tmp[4] = src[9] * src[14];
		tmp[5] = src[10] * src[13];
		tmp[6] = src[8] * src[15];
		tmp[7] = src[11] * src[12];
		tmp[8] = src[8] * src[14];
		tmp[9] = src[10] * src[12];
		tmp[10] = src[8] * src[13];
		tmp[11] = src[9] * src[12];

		// Calculate first 8 elements (cofactors)
		mat[0][0] = tmp[0] * src[5] + tmp[3] * src[6] + tmp[4] * src[7];
		mat[0][0] -= tmp[1] * src[5] + tmp[2] * src[6] + tmp[5] * src[7];
		mat[0][1] = tmp[1] * src[4] + tmp[6] * src[6] + tmp[9] * src[7];
		mat[0][1] -= tmp[0] * src[4] + tmp[7] * src[6] + tmp[8] * src[7];
		mat[0][2] = tmp[2] * src[4] + tmp[7] * src[5] + tmp[10] * src[7];
		mat[0][2] -= tmp[3] * src[4] + tmp[6] * src[5] + tmp[11] * src[7];
		mat[0][3] = tmp[5] * src[4] + tmp[8] * src[5] + tmp[11] * src[6];
		mat[0][3] -= tmp[4] * src[4] + tmp[9] * src[5] + tmp[10] * src[6];
		mat[1][0] = tmp[1] * src[1] + tmp[2] * src[2] + tmp[5] * src[3];
		mat[1][0] -= tmp[0] * src[1] + tmp[3] * src[2] + tmp[4] * src[3];
		mat[1][1] = tmp[0] * src[0] + tmp[7] * src[2] + tmp[8] * src[3];
		mat[1][1] -= tmp[1] * src[0] + tmp[6] * src[2] + tmp[9] * src[3];
		mat[1][2] = tmp[3] * src[0] + tmp[6] * src[1] + tmp[11] * src[3];
		mat[1][2] -= tmp[2] * src[0] + tmp[7] * src[1] + tmp[10] * src[3];
		mat[1][3] = tmp[4] * src[0] + tmp[9] * src[1] + tmp[10] * src[2];
		mat[1][3] -= tmp[5] * src[0] + tmp[8] * src[1] + tmp[11] * src[2];

		// Calculate pairs for second 8 elements (cofactors)
		tmp[0] = src[2] * src[7];
		tmp[1] = src[3] * src[6];
		tmp[2] = src[1] * src[7];
		tmp[3] = src[3] * src[5];
		tmp[4] = src[1] * src[6];
		tmp[5] = src[2] * src[5];
		tmp[6] = src[0] * src[7];
		tmp[7] = src[3] * src[4];
		tmp[8] = src[0] * src[6];
		tmp[9] = src[2] * src[4];
		tmp[10] = src[0] * src[5];
		tmp[11] = src[1] * src[4];

		// Calculate second 8 elements (cofactors)
		mat[2][0] = tmp[0] * src[13] + tmp[3] * src[14] + tmp[4] * src[15];
		mat[2][0] -= tmp[1] * src[13] + tmp[2] * src[14] + tmp[5] * src[15];
		mat[2][1] = tmp[1] * src[12] + tmp[6] * src[14] + tmp[9] * src[15];
		mat[2][1] -= tmp[0] * src[12] + tmp[7] * src[14] + tmp[8] * src[15];
		mat[2][2] = tmp[2] * src[12] + tmp[7] * src[13] + tmp[10] * src[15];
		mat[2][2] -= tmp[3] * src[12] + tmp[6] * src[13] + tmp[11] * src[15];
		mat[2][3] = tmp[5] * src[12] + tmp[8] * src[13] + tmp[11] * src[14];
		mat[2][3] -= tmp[4] * src[12] + tmp[9] * src[13] + tmp[10] * src[14];
		mat[3][0] = tmp[2] * src[10] + tmp[5] * src[11] + tmp[1] * src[9];
		mat[3][0] -= tmp[4] * src[11] + tmp[0] * src[9] + tmp[3] * src[10];
		mat[3][1] = tmp[8] * src[11] + tmp[0] * src[8] + tmp[7] * src[10];
		mat[3][1] -= tmp[6] * src[10] + tmp[9] * src[11] + tmp[1] * src[8];
		mat[3][2] = tmp[6] * src[9] + tmp[11] * src[11] + tmp[3] * src[8];
		mat[3][2] -= tmp[10] * src[11] + tmp[2] * src[8] + tmp[7] * src[9];
		mat[3][3] = tmp[10] * src[10] + tmp[4] * src[8] + tmp[9] * src[9];
		mat[3][3] -= tmp[8] * src[9] + tmp[11] * src[10] + tmp[5] * src[8];

		// Calculate determinant
		let det = 1.0f32 / (src[0] * mat[0][0] + src[1] * mat[0][1] + src[2] * mat[0][2] + src[3] * mat[0][3]);
		for i in 0..4 {
			for j in 0..4 {
				mat[i][j] = mat[i][j] * det;
			}
		}
		mat
	}

	pub fn clone(self) -> Mat4 {
		let [a, b, c, d] = self.rows;
		Mat4::new(&[
			a.x, a.y, a.z, a.w,
			b.x, b.y, b.z, b.w,
			c.x, c.y, c.z, c.w,
			d.x, d.y, d.z, d.w
		])
	}
}

impl Mul<Mat4> for Mat4 {
	type Output = Mat4;
	fn mul(self, rhs: Mat4) -> Mat4 {
		let mut d = [0.0f32; 16];
		let ot = rhs.transposed();

		for j in 0..4 {
			for i in 0..4 {
				d[i + j * 4] = self.rows[j].dot(ot.rows[i]);
			}
		}

		Mat4::new(&d)
	}
}

impl Mul<Vec4> for Mat4 {
	type Output = Vec4;
	fn mul(self, rhs: Vec4) -> Vec4 {
		Vec4::new(
			self.rows[0].dot(rhs),
			self.rows[1].dot(rhs),
			self.rows[2].dot(rhs),
			self.rows[3].dot(rhs),
		)
	}
}

impl Mul<Vec3> for Mat4 {
	type Output = Vec3;
	fn mul(self, rhs: Vec3) -> Vec3 {
		let v = rhs.extend(1.0);
		Vec3::new(
			self.rows[0].dot(v),
			self.rows[1].dot(v),
			self.rows[2].dot(v)
		)
	}
}

impl Mul<f32> for Mat4 {
	type Output = Mat4;
	fn mul(self, rhs: f32) -> Mat4 {
		Mat4::from_rows(
			self.rows[0] * rhs,
			self.rows[1] * rhs,
			self.rows[2] * rhs,
			self.rows[3] * rhs
		)
	}
}