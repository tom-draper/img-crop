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

fn get_args() -> Args {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Error: No image path specified.")
    }

    let path = args[1].to_owned();
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

fn crop_values(args: &Args, width: u32, height: u32) -> crop::CropValues {
    let top = clean_arg(&args.top, height);
    let right = clean_arg(&args.right, width);
    let bottom = clean_arg(&args.bottom, height);
    let left = clean_arg(&args.left, width);

    crop::CropValues::new(top, right, bottom, left)
}

fn main() {
    let args = get_args();

    if args.top == "0" && args.right == "0" && args.bottom == "0" && args.left == "0" {
        println!("No crop operations specified.\nCopying image unchanged.");
    }

    let path = Path::new(&args.path);
    let img = image::open(path).unwrap();
    let (width, height) = img.dimensions();
    
    // Extract pixel crop value from str args
    let crop_values = crop_values(&args, width, height);

    crop::run(img, path, &crop_values, &args.output);
}
