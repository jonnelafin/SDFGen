// Main image parsing and manipulation
extern crate image;
use image::GenericImageView;
use image::GenericImage;

//constants
pub const WHITE: image::Rgba<u8> = image::Rgba([255u8, 255u8, 255u8, 255u8]);
pub const INF: u32 = 9999999;

/// Returns a vector of the coordinates in a given image.
pub fn get_active(img: &image::DynamicImage) -> Vec<[u32;2]>{
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

/// Finds the closest coordinate in the give coordinate list _active_.
/// Returns and array of the size of 3. If secondpass is not enabled these will all be the same ( the closest coordinate).
/// _second\_pass_ enables the search of three closest values, instead of just one.
pub fn get_closest(active: &Vec<[u32; 2]>, value: &[u32; 2], second_pass: bool) -> [([u32; 2], u32); 3]{
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

/// Creates a black image with the dimensions of _img_.
pub fn create_shadow_copy(img: &image::DynamicImage) -> image::DynamicImage{
	let w = img.dimensions().0;
	let h = img.dimensions().1;
	image::DynamicImage::new_rgb8(w,h)
}

/// Generates a distance field from an image and an active map (obtained from get_active()).
///
/// _img_ is the input image,
///
/// _active_ the white pixels in the image,
///
/// _from_ coordinates where the reading of _img_ starts
///
/// _to_ coordinates where the reading of _img_ ends
///
/// _id_ simply an identifier for the console log. If this is not important to you just pass 0,
///
/// _scale_ the factor of the image read
///
/// The mapping of the output is all over the place.
pub fn gen_sdf(img: &image::DynamicImage, active: &Vec<[u32; 2]>, from: [u32; 2], to: [u32; 2], id: u32, scale: u32) -> image::DynamicImage{
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

