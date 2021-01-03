#[macro_use]
extern crate impl_ops;

use std::fs::File;
use std::io::BufWriter;
use std::ops;
use std::path::Path;

#[derive(Clone, Copy)]
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

impl std::ops::Index<usize> for Vec3 {
	type Output = f64;

	fn index(&self, index: usize) -> &f64 {
		&self.0[index]
	}
}

impl std::ops::AddAssign for Vec3 {
	fn add_assign(&mut self, other: Self) {
		*self = Vec3::new_with(self[0] + other[0], self[1] + other[1], self[2] + other[2]);
	}
}

impl std::ops::MulAssign<f64> for Vec3 {
	fn mul_assign(&mut self, t: f64) {
		*self = Vec3::new_with(self[0] * t, self[1] * t, self[2] * t);
	}
}

impl std::ops::DivAssign<f64> for Vec3 {
	fn div_assign(&mut self, t: f64) {
		*self *= 1.0 / t;
	}
}

impl_op_ex!(+|a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new_with(a[0] + b[0], a[1] + b[1], a[2] + b[2])});
impl_op_ex!(-|a: &Vec3, b: &Vec3| -> Vec3 {
	Vec3::new_with(a[0] - b[0], a[1] - b[1], a[2] - b[2])
});
impl_op_ex!(-|a: &Vec3| -> Vec3 { Vec3::new_with(-a[0], -a[1], -a[2]) });
impl_op_ex!(*|a: &Vec3, b: &Vec3| -> Vec3 {
	Vec3::new_with(a[0] * b[0], a[1] * b[1], a[2] * b[2])
});
impl_op_ex_commutative!(*|a: &Vec3, b: f64| -> Vec3 {
	Vec3::new_with(a[0] * b, a[1] * b, a[2] * b)
});
impl_op_ex!(/|a: &Vec3, b: f64| -> Vec3 { a * (1.0 / b) });

fn dot(u: &Vec3, v: &Vec3) -> f64 {
	u[0] * v[0] + u[1] * v[1] + u[2] * v[2]
}

fn cross(u: Vec3, v: Vec3) -> Vec3 {
	Vec3::new_with(
		u[1] * v[2] - u[2] * v[1],
		u[2] * v[0] - u[0] * v[2],
		u[0] * v[1] - u[1] * v[0],
	)
}

fn unit_vector(v: Vec3) -> Vec3 {
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

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
	let oc = &ray.origin - center;
	let a = dot(&ray.direction, &ray.direction);
	let b = 2.0 * dot(&oc, &ray.direction);
	let c = dot(&oc, &oc) - radius * radius;
	let discriminant = b * b - 4.0 * a * c;
	if discriminant < 0.0 {
		-1.0
	} else {
		(-b - discriminant.sqrt()) / (2.0 * a)
	}
}

fn ray_color(r: &Ray) -> Color {
	let t = hit_sphere(&Point3::new_with(0.0, 0.0, -1.0), 0.5, r);
	if t > 0.0 {
		let n = unit_vector(&r.at(t) - &Vec3::new_with(0.0, 0.0, -1.0));
		return 0.5 * Color::new_with(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
	}
	let unit_direction = unit_vector(r.direction);
	let t = 0.5 * (unit_direction.y() + 1.0);
	(1.0 - t) * Color::new_with(1.0, 1.0, 1.0) + t * Color::new_with(0.5, 0.7, 1.0)
}

fn main() {
	// Image
	const ASPECT_RATIO: f64 = 16.0 / 9.0;
	const IMAGE_WIDTH: usize = 400;
	const IMAGE_HEIGHT: usize = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as usize;

	// Camera
	let viewport_height = 2.0;
	let viewport_width = ASPECT_RATIO * viewport_height;
	let focal_length = 1.0;

	let origin = Point3::new();
	let horizontal = Vec3::new_with(viewport_width, 0.0, 0.0);
	let vertical = Vec3::new_with(0.0, viewport_height, 0.0);
	let lower_left_corner =
		&origin - (&horizontal / 2.0) - (&vertical / 2.0) - Vec3::new_with(0.0, 0.0, focal_length);

	// Render

	let mut pixels = Vec::with_capacity(IMAGE_WIDTH * IMAGE_HEIGHT);

	for j in (0..IMAGE_HEIGHT).rev() {
		eprint!("\rScanlines remaining: {0}       ", j);
		for i in 0..IMAGE_WIDTH {
			let u = (i as f64) / (IMAGE_WIDTH as f64 - 1.0);
			let v = (j as f64) / (IMAGE_HEIGHT as f64 - 1.0);
			let r = Ray::new_with(
				origin.clone(),
				&lower_left_corner + (u * &horizontal) + (v * &vertical) - &origin,
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

	let mut encoder = png::Encoder::new(w, IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);
	encoder.set_color(png::ColorType::RGB);
	encoder.set_depth(png::BitDepth::Eight);
	let mut writer = encoder.write_header().unwrap();

	writer.write_image_data(&pixels).unwrap();
}
