use std::path::Path;
use regex::Regex;
use image::GenericImageView;

fn clean_arg(arg: &str, dim: u32) -> u32 {
    let mut value: u32 = 0;
    let re_per = Regex::new(r"^\d+%").unwrap();
    let re_em = Regex::new(r"^\d+em").unwrap();
    let re_pix = Regex::new(r"^\d+(px)?").unwrap();
    if re_per.is_match(arg) {
        // If percentage given, calc value relative to image dimension
        value = arg.replace("%", "").parse::<u32>().unwrap();
        value *= dim / 100;
    } else if re_em.is_match(arg) {
        value = arg.replace("em", "").parse::<u32>().unwrap();
        value *= 16;  // Convert to pixels
    } else if re_pix.is_match(arg) {
        value = arg.replace("px", "").parse::<u32>().unwrap();
    }
    value
}

pub fn run(path: &str, top: &str, right: &str, bottom: &str, left: &str, output: &str) {
    let mut img = image::open(&Path::new(path)).unwrap();
    let (width, height) = img.dimensions();
    
    let top = clean_arg(top, height);
    let right = clean_arg(right, width);
    let bottom = clean_arg(bottom, height);
    let left = clean_arg(left, width);

    let cropped = img.crop(left,top, width-left-right, height-top-bottom);
    
    let mut path_split: Vec<&str> = path.split(&['.', '/', '\\'][..]).collect();
    let ext = path_split.pop().unwrap();
    let filename = path_split.pop().unwrap();
    
    let new_path: String;
    if output == "" {
        new_path = path_split.join("/") + "/" + filename + "_cropped." + ext;
    } else {
        new_path = output.to_owned()
    }
    let (new_width, new_height) = cropped.dimensions();
    println!("{}x{} -> {}x{}", width, height, new_width, new_height);
    
    println!("💾 {}", new_path);
    cropped.save(new_path).unwrap();
}