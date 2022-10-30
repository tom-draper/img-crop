extern crate image;
use clap;
use clap::Parser;

mod crop;

/// Simple program to greet a person
#[derive(Parser, Default, Debug)]
#[clap(author = "Tom Draper", about = "A Fast Image Cropping Tool")]
struct Args {
    #[arg(short, value_name="file path")]
    input: String,
    #[arg(short, long, default_value_t = 0.to_string())]
    top: String,
    #[arg(short, long, default_value_t = 0.to_string())]
    right: String,
    #[arg(short, long, default_value_t = 0.to_string())]
    bottom: String,
    #[arg(short, long, default_value_t = 0.to_string())]
    left: String,
    #[arg(short, default_value_t = String::new())]
    output: String,
}

fn main() {
    let args = Args::parse();
    let path = &*args.input;
    let top = &*args.top;
    let right = &*args.right;
    let bottom = &*args.bottom;
    let left = &*args.left;
    let output = &*args.output;
    if top == "0" && right == "0" && bottom == "0" && left == "0" {
        println!("No crop operations specified.\nCopying image unchanged.");
    }
    crop::run(path, top, right, bottom, left, output);
}
