use std::path::Path;
use image::{DynamicImage, GenericImageView};
pub struct CropValues {
    top: u32,
    right: u32,
    bottom: u32,
    left: u32,
}

impl CropValues {
  pub fn new(top: u32, left: u32, bottom: u32, right: u32) -> CropValues {
    CropValues {top, left, bottom, right}
  }
}

fn default_output_path_str(path: &Path) -> String {
    // Build output image file path with "_cropped" appended
    let ext = path.extension().unwrap().to_str().unwrap();
    let filename = path.file_name().unwrap().to_str().unwrap();
    let filestem = path.file_stem().unwrap().to_str().unwrap();
    let new_file = filestem.to_owned() + "_cropped." + ext;
    path.to_str().unwrap().replace(filename, &new_file)
}

pub fn run(img: DynamicImage, path: &Path, crop_values: &CropValues, output: &str) {
    let (width, height) = img.dimensions();
    let cropped = img.crop_imm(
        crop_values.left, 
        crop_values.top, 
        width-crop_values.left-crop_values.right, 
        height-crop_values.top-crop_values.bottom
    );
    
    // Build path for cropped image file
    let new_path_str: String;
    if output == "" {
        new_path_str = default_output_path_str(path);
    } else {
        new_path_str = output.to_owned();
    }
    let new_path = Path::new(&new_path_str);

    let (new_width, new_height) = cropped.dimensions();
    println!("{}x{} -> {}x{}", width, height, new_width, new_height);
    
    println!("💾 {:?}", new_path);
    cropped.save(new_path).unwrap();
}