// use image::{ImageReader, GenericImageView, DynamicImage};
use image::{DynamicImage, GenericImage, GenericImageView, ImageReader};

use indicatif::ProgressBar;
use rayon::prelude::*;
use std::ops::{DerefMut};
use std::{env, path::PathBuf};
use std::sync::Mutex;
mod palette;

fn main() {
    // load image

    let path = PathBuf::from(env::args().nth(1).expect("No path given"));
    println!("Loading Image {:?}", path.file_name().unwrap());
    let image_reader = ImageReader::open(&path).expect("Image couldn't be opened");
    let mut image = image_reader.decode().expect("Image couldn't be decoded");
    let palette = palette::gruvbox_palette();
    println!("Modifying Image...");
    image = modify_image(&palette, image);
    println!("Saving Image...");
    // save image
    let parent_path = &path
        .parent()
        .expect("Parent path couldn't be resolved")
        .to_str()
        .expect("Parent path couldn't be cast to string");
    let file_name = &path
        .file_name()
        .expect("File name couldn't be resolved")
        .to_str()
        .expect("Filename couldn't be cast to string");

    let new_path = format!("{}/{}_{}", parent_path, palette.name, file_name);

    
    image.save(&new_path).expect("Couldn't save image");
    println!("Image saved as \"{}\"", new_path);
}

fn modify_image(palette: &palette::Palette, image: DynamicImage)->DynamicImage{
    let clone = image.clone();
    let (width, heigth) = image.dimensions();
    let image_mutex = Mutex::new(image);
    let progress_bar = ProgressBar::new({width*heigth} as u64);

    clone
        .pixels()
        .par_bridge()
        .for_each(|original_pixel| {
            let (x, y, pixel) = original_pixel;
            let modified_pixel = palette.closest_to(&pixel);
            let mut image= image_mutex.lock().unwrap();
            image.deref_mut().put_pixel(x, y, modified_pixel);
            progress_bar.inc(1);
        });
    progress_bar.finish();
    return image_mutex.into_inner().unwrap();
}