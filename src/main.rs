use std::fs::File;
use std::io::BufWriter;
use std::ops::{AddAssign, DivAssign, Index, MulAssign, Neg};
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
	fn neg(self) -> Self::Output {
		Vec3::new_with(-self[0], -self[1], -self[2])
	}
}

impl Index<usize> for Vec3 {
	type Output = f64;

	fn index(&self, index: usize) -> &Self::Output {
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

type Point3 = Vec3;
type Color = Vec3;

fn main() {
	// Image
	let image_width = 256;
	let image_height = 256;

	// Render

	let mut pixels = Vec::with_capacity(image_width * image_height);

	for j in (0..image_height).rev() {
		eprint!("\rScanlines remaining: {0}       ", j);
		for i in 0..image_width {
			let r = (i as f64) / ((image_width - 1) as f64);
			let g = (j as f64) / ((image_height - 1) as f64);
			let b = 0.25;
			let ir = (r * 255.999) as u8;
			let ig = (g * 255.999) as u8;
			let ib = (b * 255.999) as u8;
			pixels.push(ir);
			pixels.push(ig);
			pixels.push(ib);
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
