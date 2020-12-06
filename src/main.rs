use std::fs::File;
use std::io::BufWriter;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub};
use std::path::Path;

#[derive(Clone)]
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
	fn x(&self) -> f64 {
		self[0]
	}
	fn y(&self) -> f64 {
		self[1]
	}
	fn z(&self) -> f64 {
		self[2]
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

impl Sub for &Vec3 {
	type Output = Vec3;
	fn sub(self, other: &Vec3) -> Vec3 {
		Vec3::new_with(self[0] - other[0], self[1] - other[1], self[2] - other[2])
	}
}

impl Sub<Vec3> for &Vec3 {
	type Output = Vec3;
	fn sub(self, other: Vec3) -> Vec3 {
		Vec3::new_with(self[0] - other[0], self[1] - other[1], self[2] - other[2])
	}
}

impl Sub<&Vec3> for Vec3 {
	type Output = Vec3;
	fn sub(self, other: &Vec3) -> Vec3 {
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

impl Mul<&Vec3> for f64 {
	type Output = Vec3;
	fn mul(self, vec: &Vec3) -> Vec3 {
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
	origin: Point3,
	direction: Vec3,
}

impl Ray {
	fn new() -> Ray {
		Ray {
			origin: Point3::new(),
			direction: Vec3::new(),
		}
	}
	fn new_with(origin: Point3, direction: Vec3) -> Ray {
		Ray { origin, direction }
	}
	fn at(&self, t: f64) -> Point3 {
		return &self.origin + (&self.direction * t);
	}
}

fn ray_color(r: &Ray) -> Color {
	let unit_direction = unit_vector(&r.direction);
	let t = 0.5 * (unit_direction.y() + 1.0);
	(1.0 - t) * Color::new_with(1.0, 1.0, 1.0) + t * Color::new_with(0.5, 0.7, 1.0)
}

fn main() {
	// Image
	const aspect_ratio: f64 = 16.0 / 9.0;
	const image_width: usize = 400;
	const image_height: usize = ((image_width as f64) / aspect_ratio) as usize;

	// Camera
	let viewport_height = 2.0;
	let viewport_width = aspect_ratio * viewport_height;
	let focal_length = 1.0;

	let origin = Point3::new();
	let horizontal = Vec3::new_with(viewport_width, 0.0, 0.0);
	let vertical = Vec3::new_with(0.0, viewport_height, 0.0);
	let lower_left_corner =
		&origin - &horizontal / 2 - &vertical / 2 - Vec3::new_with(0.0, 0.0, focal_length);

	// Render

	let mut pixels = Vec::with_capacity(image_width * image_height);

	for j in (0..image_height).rev() {
		eprint!("\rScanlines remaining: {0}       ", j);
		for i in 0..image_width {
			let u = (i as f64) / (image_width as f64 - 1.0);
			let v = (j as f64) / (image_height as f64 - 1.0);
			let r = Ray::new_with(
				origin.clone(),
				&lower_left_corner + u * &horizontal + v * &vertical - &origin,
			);

			let pixel_color = ray_color(&r);
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
