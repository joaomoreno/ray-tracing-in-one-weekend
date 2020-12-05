use std::path::Path;
use std::fs::File;
use std::io::BufWriter;



fn main() {
	// Image
	
	let image_width = 256;
	let image_height = 256;

	// Render

	let mut pixels = Vec::with_capacity(image_width * image_height);

	for j in (0..image_height).rev() {
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