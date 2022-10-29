# Image Crops

A Rust-based CLI tool to perform fast image crops.

## Installation

```bash
git clone https://github.com/tom-draper/img-crop.git
cd img-crop
cargo build --release
```

## Usage

The image path must be specified each time using the -i flag. The cropping of specific sides can be specified with the -t, -b, -l and -r flags. Crop values can be specified as a percentage, or as an amount of pixels with an optional "px" postfix.

The command below would crop 10 pixels off the top, 15 pixels of off the right and 20% of the left of the image.

```bash
imgcrop -i <img_path> -t 10px -r 15 -l 20%
```

An output path can be specified with the -o flag. The output path defaults to the same directory as the input image.

```bash
imgcrop -i <img_path> -r <right> -o <output_path>
```
