use anyhow::{bail, Result};
use image::{DynamicImage, GenericImage, GenericImageView, Rgb, Rgba};
use indicatif::ProgressBar;
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::{
    fs::File, io::{self, BufRead}, iter::zip, path::Path, sync::Mutex
};
use thiserror::Error;


#[derive(Debug, Clone)]
pub struct Palette {
    pub colors: Vec<Rgb<u8>>,
}

#[derive (Debug, Error)]
enum PaletteErrors{
    #[error("")]
    ParseRGBError,

    #[error("")]
    NotEnoughDataError,
}

impl Palette {
    pub fn closest_to<'a>(&'a self, color: &Rgba<u8>) -> Rgba<u8> {
        closest_color(color, &self.colors)
    }
}

fn closest_color<'a>(color: &Rgba<u8>, colors: &'a Vec<Rgb<u8>>) -> Rgba<u8> {
    let mut closest: &Rgb<u8> = colors.get(0).unwrap();
    let mut closest_distance: f64 = distance(color, closest);
    for c in colors {
        let current_distance: f64 = distance(color, c);
        if current_distance < closest_distance {
            closest = &c;
            closest_distance = current_distance;
        }
    }

    return Rgba([closest[0], closest[1], closest[2], color[3]]);
}

fn distance(c1: &Rgba<u8>, c2: &Rgb<u8>) -> f64 {
    let delta_r = if c2.0[0] > c1.0[0] {
        ((c2.0[0] - c1.0[0]) as u64).pow(2)
    } else {
        ((c1.0[0] - c2.0[0]) as u64).pow(2)
    };
    let delta_g = if c2.0[1] > c1.0[1] {
        ((c2.0[1] - c1.0[1]) as u64).pow(2)
    } else {
        ((c1.0[1] - c2.0[1]) as u64).pow(2)
    };
    let delta_b = if c2.0[2] > c1.0[2] {
        ((c2.0[2] - c1.0[2]) as u64).pow(2)
    } else {
        ((c1.0[2] - c2.0[2]) as u64).pow(2)
    };

    f64::sqrt((delta_r + delta_g + delta_b) as f64)
}

pub fn change_image_palette(palette: &Palette, image: DynamicImage) -> anyhow::Result<DynamicImage> {
    let clone = image.clone();
    let (width, heigth) = image.dimensions();
    let image_mutex = Mutex::new(image);
    let progress_bar = ProgressBar::new({ width * heigth } as u64);

    clone.pixels().par_bridge().for_each(|original_pixel| {
        let (x, y, pixel) = original_pixel;
        let modified_pixel = palette.closest_to(&pixel);
        let mut image = image_mutex.lock();
        image
            .as_deref_mut()
            .expect("Image Mutex couldnt be dereferenced")
            .put_pixel(x, y, modified_pixel);
        progress_bar.inc(1);
    });
    progress_bar.finish();
    let final_image = image_mutex.into_inner();
    match final_image {
        Ok(im) => Ok(im),
        Err(e) => Err(e.into()),
    }
}

pub fn parse_palette(path: &Path) -> anyhow::Result<Palette> {
    let mut new_palette = Palette {
        colors: vec![],
    };

    let mut file_lines = read_lines(path)?;

    while let Some(possible_line) = file_lines.next() {
        if possible_line.is_err() {
            break;
        }

        let curr_line = possible_line?;
        new_palette.colors.push(str_to_rgb(curr_line)?);
    }

    if new_palette.colors.len() == 0 {
        Err(PaletteErrors::NotEnoughDataError.into())
    } else {
        Ok(new_palette)
    }
}

fn read_lines<P>(filename: P) -> Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn str_to_rgb(str: String) -> anyhow::Result<Rgb<u8>> {
    let mut new_rgb = Rgb([0u8, 0u8, 0u8]);

    let iter: Vec<&str> = str.split_ascii_whitespace().collect();

    if iter.len() != 3 {
        bail!(PaletteErrors::ParseRGBError)
    }

    for (val, i) in zip(iter, 0..3){
        new_rgb[i] = val.parse::<u8>()?;
    }

    Ok(new_rgb)
}
