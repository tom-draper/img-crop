extern crate image;
use std::path::Path;
use regex::Regex;
use image::GenericImageView;
use std::env;

mod crop;

/// Simple program to greet a person
#[derive(Default, Debug)]
struct Args {
    path: String,
    top: String,
    right: String,
    bottom: String,
    left: String,
    output: String,
}

fn valid_path(path: String) -> bool {
    let _path = Path::new(&path);
    let _ = _path.file_name();
    let _ = _path.extension();
    return true
}

fn get_args() -> Args {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Error: No image path specified.")
    }

    let path = args[1].to_owned();

    if !valid_path(path.clone()) {
        panic!("Error: Image path invalid.")
    }

    let mut top = String::from("0");
    let mut right = String::from("0");
    let mut bottom = String::from("0");
    let mut left = String::from("0");
    let mut output = String::new();
    for (i, arg) in args.iter().enumerate() {
        if arg == "-t" || arg == "--top" || arg == "-top" {
            top = args[i+1].to_owned();
        } else if arg == "-r" || arg == "--right" || arg == "-right" && i < args.len()-1 {
            right = args[i+1].to_owned();
        } else if arg == "-b" || arg == "--bottom" || arg == "-bottom"  && i < args.len()-1 {
            bottom = args[i+1].to_owned();
        } else if arg == "-l" || arg == "--left" || arg == "-left"  && i < args.len()-1 {
            left = args[i+1].to_owned();
        } else if arg == "-o" || arg == "--output" || arg == "-output"  && i < args.len()-1 {
            output = args[i+1].to_owned();
        }
    }

    Args {path, top, left, bottom, right, output}
}

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

fn default_output_path_str(path: &Path) -> String {
    // Build output image file path with "_cropped" appended
    let ext = path.extension().unwrap().to_str().unwrap();
    let filename = path.file_name().unwrap().to_str().unwrap();
    let filestem = path.file_stem().unwrap().to_str().unwrap();
    let new_file = filestem.to_owned() + "_cropped." + ext;
    path.to_str().unwrap().replace(filename, &new_file)
}

fn output_path_str(path: &Path, output: &str) -> String {
    let new_path_str: String;
    if output == "" {
        new_path_str = default_output_path_str(path);
    } else {
        new_path_str = output.to_owned();
    }
    new_path_str
}

fn crop_values(args: &Args, width: u32, height: u32) -> crop::CropValues {
    let top = clean_arg(&args.top, height);
    let right = clean_arg(&args.right, width);
    let bottom = clean_arg(&args.bottom, height);
    let left = clean_arg(&args.left, width);

    crop::CropValues::new(top, right, bottom, left)
}

fn crop_image(args: Args) {
    let path = Path::new(&args.path);

    let img = image::open(path).unwrap();
    let (width, height) = img.dimensions();
    
    // Build path for cropped image file
    let output_path_str = output_path_str(path, &args.output);
    let output_path = Path::new(&output_path_str);
    
    // Extract pixel crop value from str args
    let crop_values = crop_values(&args, width, height);
    
    crop::run(img, &crop_values, output_path);
}

fn main() {
    let args = get_args();

    if args.top == "0" && args.right == "0" && args.bottom == "0" && args.left == "0" {
        println!("No crop operations specified.\nCopying image unchanged.");
    }

    crop_image(args);
}
