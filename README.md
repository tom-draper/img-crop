# Image Crop

A Rust-based tool to perform fast image crops from the command line.

## Installation

```bash
git clone https://github.com/tom-draper/img-crop.git
cd img-crop
cargo build --release
```

## Usage

The image path must be specified each time using the `-i` flag. Cropping is specified with an edge flag (`-t`, `-b`, `-l` or `-r`) followed by a crop value. Crop values can be a percentage, or an amount of pixels with an optional "px" postfix.

The command below would crop 10 pixels off the top, 15 pixels of off the right and 20% of the left of the image.

```bash
crop -i <img_path> -t 10px -r 15 -l 20%
```

An optional output path can be specified with the `-o` flag. This defaults to the same directory as the input image.

```bash
crop -i <img_path> -r <right> -o <output_path>
```
