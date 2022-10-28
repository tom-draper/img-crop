extern crate image; 
use std::path::Path;
use image::GenericImageView;

fn clean_arg(arg: &str, dim: u32) -> u32 {
    let mut value: u32;
    if arg.contains("%") {
        // If percentage given, calc value relative to image dimension
        value = arg.replace("%", "").parse::<u32>().unwrap();
        value *= dim / 100;
    } else {
        value = arg.parse::<u32>().unwrap();
    }
    value
}

fn run(path: &str, top: &str, right: &str, bottom: &str, left: &str) {
    let mut img = image::open(&Path::new(path)).unwrap();
    let (width, height) = img.dimensions();
    
    let top = clean_arg(top, height);
    let right = clean_arg(right, width);
    let bottom = clean_arg(bottom, height);
    println!("{}", bottom);
    let left = clean_arg(left, width);

    let cropped = img.crop(left,top, width-left-right, height-top-bottom);
    
    let mut path_split: Vec<&str> = path.split(&['.', '/', '\\'][..]).collect();
    let ext = path_split.pop().unwrap();
    let filename = path_split.pop().unwrap();
    
    let new_path: String = path_split.join("/") + "/" + filename + "_cropped." + ext;
    let (new_width, new_height) = cropped.dimensions();
    println!("{}x{} -> {}x{}", width, height, new_width, new_height);
    
    println!("💾 {}", new_path);
    cropped.save(new_path).unwrap();
}

fn main() {
    run("img/puppy.jpg", "40%", "40%", "40%", "0");
}
