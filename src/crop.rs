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

pub fn run(img: DynamicImage, crop_values: &CropValues, output_path: &Path) {
    let (width, height) = img.dimensions();
    let cropped = img.crop_imm(
        crop_values.left, 
        crop_values.top, 
        width-crop_values.left-crop_values.right, 
        height-crop_values.top-crop_values.bottom
    );

    let (new_width, new_height) = cropped.dimensions();
    println!("{}x{} -> {}x{}", width, height, new_width, new_height);
    
    println!("💾 {:?}", output_path);
    cropped.save(output_path).unwrap();
}