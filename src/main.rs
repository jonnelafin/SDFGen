// <Readme>  //
//! DO NOT USE IN PRODUCTION IF YOU ARE SANE. This package is not actively maintained and performs suboptimally to say the least.
//! ## Alternatives
//! - [distance-field](https://crates.io/crates/distance-field)
//!
//! SDFGen generates a (preferably) lower resolution distance field from an input image.
//! This distance field can be upscaled fast with bilinear filters built into gpus nowadays.
//! Final sharp output of this upscale can be extracted using a threshold filter. With the default parameters in this program, threshold should be set to about 99%. 
//!
//! # Building the project
//! ```console
//! cargo build --release
//! ```
//! Without the release flag, the execution time can be up to 10 times slower.
//! # Running the project
//! ```console
//!   ./target/release/sdfgen [OPTIONS]
//! ```
//! ```
//! Optional arguments:
//!   -h,--help             Show this help message and exit
//!   -v,--verbose          Be verbose
//!   -f,--file FILE        filepath of the input image. Defaults to
//!                         "images/sdf500.png"
//!   -s,--scale SCALE      Scale of the distance field. For example resolution
//!                         500x500 becomes 250x250 with scale 2 and 125x125 with
//!                         scale 4. Defaults to 32.
//!   -t,--threads THREADS  Number of assigned threads. Defaults to 8.
//!   -V,--version          Show version
//! ```
//! <!-- Please do not edit the README.md file directly, as it is compiled from the documentation at src/main.rs -->
//! <!-- Instead, make the edits into the main.rs file and then run the compile_readme.sh or equivalent. --> 
// </Readme> //

// Main image parsing and manipulation
extern crate image;
use image::GenericImageView;
use image::GenericImage;

// Threading
use std::thread;

// Excecution time
extern crate chrono;
use chrono::Utc;

// Argument parsing
extern crate argparse;
use argparse::{ArgumentParser, StoreTrue, Store};

// Internal lib
use sdfgen_lib::*;

//const MAX_DIST: u32 = 32;
//const scale: u32 = 32;

//const threads: usize = 16;

/// Adds two rgba pixels together.
fn join_rgba(r1: &image::Rgba<u8>, r2: &image::Rgba<u8>) -> image::Rgba<u8>{
	let r = r1[0] + r2[0];
	let g = r1[1] + r2[1];
	let b = r1[2] + r2[2];
	image::Rgba([r,g,b, 255])
}

/// Adds two images together, pixel by pixel. Dimensions of the output image is read from _img_.
/// It is a good idea for the two images to be the same size, otherwise this function will probably panic. ;)
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

/// Main program function that parses the arguments and invokes the necessary functions to generate and save the distance field.
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
