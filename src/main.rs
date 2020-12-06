use std::fs::File;
use std::io::BufWriter;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub};
use std::path::Path;

struct Vec3([f64; 3]);

impl Vec3 {
	fn new() -> Vec3 {
		Vec3 { 0: [0.0, 0.0, 0.0] }
	}
	fn new_with(x: f64, y: f64, z: f64) -> Vec3 {
		Vec3 { 0: [x, y, z] }
	}
	fn length(&self) -> f64 {
		self.length_squared().sqrt()
	}
	fn length_squared(&self) -> f64 {
		self[0].powi(2) + self[1].powi(2) + self[2].powi(2)
	}
}

impl Neg for Vec3 {
	type Output = Vec3;
	fn neg(self) -> Vec3 {
		Vec3::new_with(-self[0], -self[1], -self[2])
	}
}

impl Index<usize> for Vec3 {
	type Output = f64;

	fn index(&self, index: usize) -> &f64 {
		&self.0[index]
	}
}

impl AddAssign for Vec3 {
	fn add_assign(&mut self, other: Self) {
		*self = Vec3::new_with(self[0] + other[0], self[1] + other[1], self[2] + other[2]);
	}
}

impl MulAssign<f64> for Vec3 {
	fn mul_assign(&mut self, t: f64) {
		*self = Vec3::new_with(self[0] * t, self[1] * t, self[2] * t);
	}
}

impl DivAssign<f64> for Vec3 {
	fn div_assign(&mut self, t: f64) {
		*self *= 1.0 / t;
	}
}

trait Vec3Collection {
	fn push_vec(&mut self, vec3: &Vec3);
}

impl Vec3Collection for Vec<u8> {
	fn push_vec(&mut self, vec: &Vec3) {
		self.push((vec[0] * 255.999) as u8);
		self.push((vec[1] * 255.999) as u8);
		self.push((vec[2] * 255.999) as u8);
	}
}

impl Add for Vec3 {
	type Output = Vec3;
	fn add(self, other: Vec3) -> Vec3 {
		Vec3::new_with(self[0] + other[0], self[1] + other[1], self[2] + other[2])
	}
}

impl Add for &Vec3 {
	type Output = Vec3;
	fn add(self, other: &Vec3) -> Vec3 {
		Vec3::new_with(self[0] + other[0], self[1] + other[1], self[2] + other[2])
	}
}

impl Add<Vec3> for &Vec3 {
	type Output = Vec3;
	fn add(self, other: Vec3) -> Vec3 {
		Vec3::new_with(self[0] + other[0], self[1] + other[1], self[2] + other[2])
	}
}

impl Add<&Vec3> for Vec3 {
	type Output = Vec3;
	fn add(self, other: &Vec3) -> Vec3 {
		Vec3::new_with(self[0] + other[0], self[1] + other[1], self[2] + other[2])
	}
}

impl Sub for Vec3 {
	type Output = Vec3;
	fn sub(self, other: Vec3) -> Vec3 {
		Vec3::new_with(self[0] - other[0], self[1] - other[1], self[2] - other[2])
	}
}

impl Mul for Vec3 {
	type Output = Vec3;
	fn mul(self, other: Vec3) -> Vec3 {
		Vec3::new_with(self[0] * other[0], self[1] * other[1], self[2] * other[2])
	}
}

impl<T: Into<f64>> Mul<T> for Vec3 {
	type Output = Vec3;
	fn mul(self, t: T) -> Vec3 {
		let t = t.into();
		Vec3::new_with(self[0] * t, self[1] * t, self[2] * t)
	}
}

impl<T: Into<f64>> Mul<T> for &Vec3 {
	type Output = Vec3;
	fn mul(self, t: T) -> Vec3 {
		let t = t.into();
		Vec3::new_with(self[0] * t, self[1] * t, self[2] * t)
	}
}

impl Mul<Vec3> for f64 {
	type Output = Vec3;
	fn mul(self, vec: Vec3) -> Vec3 {
		vec * self
	}
}

impl Mul<Vec3> for u32 {
	type Output = Vec3;
	fn mul(self, vec: Vec3) -> Vec3 {
		vec * self
	}
}

impl<T: Into<f64>> Div<T> for Vec3 {
	type Output = Vec3;
	fn div(self, t: T) -> Vec3 {
		self * (1.0 / t.into())
	}
}

impl<T: Into<f64>> Div<T> for &Vec3 {
	type Output = Vec3;
	fn div(self, t: T) -> Vec3 {
		self * (1.0 / t.into())
	}
}

fn dot(u: &Vec3, v: &Vec3) -> f64 {
	u[0] * v[0] + u[1] * v[1] + u[2] * v[2]
}

fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
	Vec3::new_with(
		u[1] * v[2] - u[2] * v[1],
		u[2] * v[0] - u[0] * v[2],
		u[0] * v[1] - u[1] * v[0],
	)
}

fn unit_vector(v: &Vec3) -> Vec3 {
	v / v.length()
}

type Point3 = Vec3;
type Color = Vec3;

struct Ray {
	orig: Point3,
	dir: Vec3,
}

impl Ray {
	fn new() -> Ray {
		Ray {
			orig: Point3::new(),
			dir: Vec3::new(),
		}
	}
	fn new_with(orig: Point3, dir: Vec3) -> Ray {
		Ray { orig, dir }
	}
	fn at(&self, t: f64) -> Point3 {
		return &self.orig + (&self.dir * t);
	}
}

fn main() {
	// Image
	let image_width = 256;
	let image_height = 256;

	// Render

	let mut pixels = Vec::with_capacity(image_width * image_height);

	for j in (0..image_height).rev() {
		eprint!("\rScanlines remaining: {0}       ", j);
		for i in 0..image_width {
			let pixel_color = Color::new_with(
				(i as f64) / ((image_width - 1) as f64),
				(j as f64) / ((image_height - 1) as f64),
				0.25,
			);
			pixels.push_vec(&pixel_color);
		}
	}
	eprintln!("\nDone");

	// Write image

	let path = Path::new(r"image.png");
	let file = File::create(path).unwrap();
	let ref mut w = BufWriter::new(file);

	let mut encoder = png::Encoder::new(w, image_width as u32, image_height as u32);
	encoder.set_color(png::ColorType::RGB);
	encoder.set_depth(png::BitDepth::Eight);
	let mut writer = encoder.write_header().unwrap();

	writer.write_image_data(&pixels).unwrap();
}
