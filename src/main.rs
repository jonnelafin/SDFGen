extern crate image;

use image::GenericImageView;
use image::GenericImage;

const WHITE: image::Rgba<u8> = image::Rgba([255u8, 255u8, 255u8, 255u8]);
const INF: u32 = 9999999;
const MAX_DIST: u32 = 32;

fn get_active(img: &image::DynamicImage) -> Vec<[u32;2]>{
	let w = img.dimensions().0;
	let h = img.dimensions().1;
	let ws = w as usize;
	let hs = h as usize;
	let mut out = vec![[INF, INF];ws*hs];
	let mut ind = 0;
	let full = w*h;
	for x in 0..w {
		let xs = x as usize;
		for y in 0..h{
			let ys = y as usize;
    		let pixel = img.get_pixel(y, x);
    		if pixel == WHITE {
//    			print!("#");
    			out[ind] = [x, y];
    		}
    		else{
//    			print!("_");
    		}
    		ind += 1;
		}
		print!("[{}%]: {}/{}", ((ind as f64)/(full as f64) * 100.0) as u32, ind, full);
		println!("");
	}
	out
}

fn get_closest(active: &Vec<[u32; 2]>, value: &[u32; 2]) -> ([u32; 2], u32){
	let x = value[0] as f64;
	let y = value[1] as f64;
	let mut best = [INF, INF];
	let mut bests= INF;
	for i in active{
		let ix = i[0] as f64;
		let iy = i[1] as f64;
		let diff_x = (x - ix).abs() as u32;
		let diff_y = (y - iy).abs() as u32;
		let diff = diff_x + diff_y;
		if diff < bests {
			bests = diff;
			best = *i;
		}
	}
	return (best, bests);
}

fn gen_sdf(mut img: image::DynamicImage, active: &Vec<[u32; 2]>) -> image::DynamicImage{
	let scale = 2;
	let w = img.dimensions().0/scale;
	let h = img.dimensions().1/scale;
	let ws = w as usize;
	let hs = h as usize;
	let full = w*h;
	let max_dist = 255.0 / w as f64;
	let mut ind = 0;
	for x in 0..w {
		let xs = x as usize;
		for y in 0..h{
			let ys = y as usize;
			let r = 255u8 - (get_closest(&active, &[x*scale, y*scale]).1 as f64 * max_dist) as u8;
    		img.put_pixel(y, x, image::Rgba([r,r,r, 255]));
    		//img.put_pixel(y*scale, x*scale, image::Rgba([r,r,r, 255])); //nice effect
    		ind += 1;
		}
		print!("[{}%]: {}/{}", ((ind as f64)/(full as f64) * 100.0) as u32, ind, full);
		println!("");
	}
	img
}

fn main() {
    println!("Opening image...");
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let mut img = image::open("valve_256.png").unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());
    println!("Reading pixels...");
	let active = get_active(&img);
	println!("Closest: {:?}", get_closest(&active, &[0, 0]));
    println!("Generating sdf...");
	img = gen_sdf(img, &active);
    println!("Saving pixels...");
    // Write the contents of this image to the Writer in PNG format.
    img.save("out_sdf.png").unwrap();
    println!("Have a good day.");
}
