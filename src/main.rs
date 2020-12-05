/*--------------------------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
 *-------------------------------------------------------------------------------------------------------------*/

fn main() {

	// Image

	let image_width = 256;
	let image_height = 256;

	// Render

	print!("P3\n{0} {1}\n255\n", image_width, image_height);

	for j in (0..image_height).rev() {
		for i in 0..image_width {
			let r = (i as f64) / ((image_width - 1) as f64);
			let g = (j as f64) / ((image_height - 1) as f64);
			let b = 0.25;

			let ir = (r * 255.999) as i32;
			let ig = (g * 255.999) as i32;
			let ib = (b * 255.999) as i32;

			print!("{0} {1} {2}\n", ir, ig, ib);
		}
	}
}