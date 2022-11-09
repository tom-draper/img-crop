# Image Crop

A Rust-based tool to perform fast image cropping from the command-line.

<p align="center">
	<img src="https://user-images.githubusercontent.com/41476809/200960588-a6d3d003-a405-41d8-a048-00336b74ef57.png">
</p>

## Installation

```bash
git clone https://github.com/tom-draper/img-crop.git
cd img-crop
cargo build --release
```

## Usage

The image path must be the first command-line argument specified. Cropping is specified with an edge flag (`-t`, `-b`, `-l` or `-r`) followed by a crop value. Crop values can be a percentage, or an amount of pixels with an optional "px" postfix.

The command below would crop 10 pixels off the top, 15 pixels off the right and 20% off the left of the image.

```bash
crop <img_path> -t 10px -r 15 -l 20%
```

An optional output path can be specified with the `-o` flag. This defaults to the same directory as the input image.

```bash
crop <img_path> -r 40 -o <output_path>
```
