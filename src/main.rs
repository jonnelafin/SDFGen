// <Readme>  //
//! SDFGen generates a preferably lower resolution distance field of an input image.
//!
//! This distance field can be upscaled fast with bilinear filters built into gpus nowadays.
//! Final sharp output of this upscale can be extracted using a threshold filter. With the default parameters in this program, threshold should be set to about 99%. 
//!
//! # Examples
//! Run the project
//! ```
//! cargo run --release
//! ```
//! Without the release flag, the execution time can be up to 10 times slower.
//!
//! Currently all variables are hardcoded, they can be changed in the main program file "/scr/main.rs".
// </Readme> //

//Main image parsing and manipulation
extern crate image;
use image::GenericImageView;
use image::GenericImage;

//Threading
use std::thread;

//Excecution time
extern crate chrono;
use chrono::Utc;

//Argument parsing
extern crate argparse;
use argparse::{ArgumentParser, StoreTrue, Store};


const WHITE: image::Rgba<u8> = image::Rgba([255u8, 255u8, 255u8, 255u8]);
const INF: u32 = 9999999;
//const MAX_DIST: u32 = 32;
//const scale: u32 = 32;

//const threads: usize = 16;

fn get_active(img: &image::DynamicImage) -> Vec<[u32;2]>{
	let w = img.dimensions().0;
	let h = img.dimensions().1;
	let ws = w as usize;
	let hs = h as usize;
	let mut out = vec![[INF, INF];ws*hs];
	let mut ind = 0;
	let full = w*h;
	for x in 0..w {
		let _xs = x as usize;
		for y in 0..h{
			let _ys = y as usize;
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
		let percent = (ind as f64)/(full as f64) * 100.0;
		if percent % 10.0 == 0.0{
			print!("[{}%]: {}/{} \r", percent as u32, ind, full);
		}
	}
	println!("");
	out
}

fn get_closest(active: &Vec<[u32; 2]>, value: &[u32; 2], second_pass: bool) -> [([u32; 2], u32); 3]{
	let x = value[0] as f64;
	let y = value[1] as f64;
	let mut best = [INF, INF];
	let mut bests= INF;
	//Second pass
	let mut best_l= [INF, INF];
	let mut bests_l=INF;
	//Third pass
	let mut best_t = [INF, INF];
	let mut bests_t=INF;
	
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
		else if second_pass && diff < bests_l{
			bests_l = diff;
			best_l= *i;
		}
		else if second_pass && diff < bests_t{
			bests_t = diff;
			best_t  = *i;
		}
	}
	if second_pass{
		return [(best, bests), (best_l, bests_l), (best_t, bests_t)];
	}
	return [(best, bests), (best, bests), (best, bests)];
}

fn gen_sdf(img: &image::DynamicImage, active: &Vec<[u32; 2]>, from: [u32; 2], to: [u32; 2], id: u32, scale: u32) -> image::DynamicImage{
//	let scale = 4;
	let w = img.dimensions().0/scale;
	let h = img.dimensions().1/scale;
	let _ws = w as usize;
	let _hs = h as usize;
	
	let mut to_x = to[0];
	if to_x == 0{to_x = w;}
	let mut to_y = to[1];
	if to_y == 0{to_y = h;}
	let w2 = to_x - from[0];
	let h2 = to_y - from[1];

	let mut out = create_shadow_copy(img);
	
	let full = w2*h2;
	let max_dist = 255.0 / w2 as f64;
	let mut ind = 0;
	for x in from[0]..to_x {
		let _xs = x as usize;
		for y in from[1]..to_y{
			let _ys = y as usize;
			let closest = get_closest(&active, &[x*scale, y*scale], false);
			let r = 255u8 - (closest[0].1 as f64 / max_dist) as u8;
			let g = 255u8 - (closest[1].1 as f64 / max_dist) as u8;
			let b = 255u8 - (closest[2].1 as f64 / max_dist) as u8;
    		out.put_pixel(y, x, image::Rgba([r,g,b, 255]));
    		//img.put_pixel(y*scale, x*scale, image::Rgba([r,r,r, 255])); //nice effect
    		ind += 1;
		}
		let percent = ((ind as f64)/(full as f64) * 100.0) as u32;
		if percent % 10 == 0{
			print!("{}[{}%]: {}/{} \r", id, percent, ind, full);
		}
	}
	println!("");
	out
}
fn create_shadow_copy(img: &image::DynamicImage) -> image::DynamicImage{
	let w = img.dimensions().0;
	let h = img.dimensions().1;
	image::DynamicImage::new_rgb8(w,h)
}
fn join_rgba(r1: &image::Rgba<u8>, r2: &image::Rgba<u8>) -> image::Rgba<u8>{
	let r = r1[0] + r2[0];
	let g = r1[1] + r2[1];
	let b = r1[2] + r2[2];
	image::Rgba([r,g,b, 255])
}
fn join_images(img: &image::DynamicImage, img2: &image::DynamicImage) -> image::DynamicImage{
	let mut img3 = create_shadow_copy(img);
	let w = img.dimensions().0;
	let h = img.dimensions().1;
	for x in 0..w {
		let _xs = x as usize;
		for y in 0..h{
			let _ys = y as usize;
			let p1 = img.get_pixel(y, x);
			let p2 = img2.get_pixel(y, x);
			let r = join_rgba(&p1, &p2);
    		img3.put_pixel(y, x, r);
		}
	}
	img3
}

fn main() {
    let mut verbose = false;

    let mut filename = "images/sdf500.png".to_string();
    let mut threads = 4;
    let mut scale = 32;
	{  // this block limits scope of borrows by ap.refer() method
		let mut ap = ArgumentParser::new();
		ap.set_description("Generates a distance field from any given image.");
		ap.refer(&mut verbose)
			.add_option(&["-v", "--verbose"], StoreTrue,
			"Be verbose");
		ap.refer(&mut filename)
			.add_option(&["-f", "--file"], Store,
			"filepath of the input image. Defaults to \"images/sdf500.png\"");
		ap.refer(&mut scale)
			.add_option(&["-s" ,"--scale"], Store,
			"Scale of the distance field. For example resolution 500x500 becomes 250x250 with scale 2 and 125x125 with scale 4. Defaults to 32.");
		ap.refer(&mut threads)
			.add_option(&["-t" ,"--threads"], Store,
			"Number of assigned threads. Defaults to 8.");
		ap.add_option(&["-V", "--version"],
		    argparse::Print(env!("CARGO_PKG_VERSION").to_string()), "Show version");
		ap.parse_args_or_exit();
	}
    let start_time = Utc::now().time();
    println!("Opening image...");
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open(filename).unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());
    println!("Reading pixels...");
	let active = get_active(&img);
	println!("Closest: {:?}", get_closest(&active, &[0, 0], true));
    println!("Generating sdf...");
	let mut img3 = create_shadow_copy(&img);
    
    let mut handles= vec![];
    for i in 0..threads{
    	let i2 = i as u32;
	    let img_clone = img.clone();
	    let active_clone = active.clone();
	    let handle = thread::spawn(move || {
			let w = img_clone.dimensions().0/scale;
			let _h = img_clone.dimensions().1/scale;
			gen_sdf(&img_clone, &active_clone, [0,((w/threads as u32)*(i2))], [0,((w/threads as u32)*(i2+1))], i2, scale)
		});
		handles.push(handle);
//	    let handle2 = thread::spawn(move || {
//			let w = img_clone.dimensions().0/scale;
//			let h = img_clone.dimensions().1/scale;
//			gen_sdf(&img_clone, &active_clone, [0,w/2], [0,0])
//		});
	}
	for _ in 0..threads{
		let result = handles.pop();
//		let backup = img3.clone();
		img3 = match result{
			Some(x) => join_images(&(x.join().unwrap()), &img3),
			None => img3
		};
//		let img2 = handles.pop()
		
	}
	println!("Cropping...");
	let w = img3.dimensions().0/scale;
	let h = img3.dimensions().1/scale;
	let fin = image::DynamicImage::crop(&mut img3, 0, 0, w, h);
    println!("Saving pixels...");
    // Write the contents of this image to the Writer in PNG format.
    fin.save("out_sdf.png").unwrap();
    let end_time = Utc::now().time();
    let diff = end_time - start_time;
    println!("Took {}ms to execute.", diff.num_milliseconds());
    println!("Have a good day.");
}
