use std::{sync::Mutex};
use image::{DynamicImage, GenericImage, GenericImageView, Rgb, Rgba};
use indicatif::ProgressBar;
use rayon::iter::{ParallelBridge, ParallelIterator};
pub struct Palette {
    pub name: &'static str,
    pub colors: Vec<Rgb<u8>>
}

impl Palette {
    pub fn closest_to<'a>(&'a self, color: &Rgba<u8>)->Rgba<u8>{
        closest_color(color, &self.colors)
    }
}

fn closest_color<'a>(color: &Rgba<u8>, colors: &'a Vec<Rgb<u8>>)->Rgba<u8>{
    let mut closest: &Rgb<u8> = colors.get(0).unwrap();
    let mut closest_distance: f64 = distance(color, closest);
    for c in colors{
        let current_distance: f64 = distance(color, c);
        if current_distance < closest_distance {
            closest = &c;
            closest_distance = current_distance;
        }
    }

    return Rgba([closest[0], closest[1], closest[2], color[3]]);
}

fn distance(c1: &Rgba<u8>, c2: &Rgb<u8>)->f64{
    let delta_r = if c2.0[0] > c1.0[0] {((c2.0[0] - c1.0[0]) as u64).pow(2)} else {((c1.0[0] - c2.0[0]) as u64).pow(2)};
    let delta_g = if c2.0[1] > c1.0[1] {((c2.0[1] - c1.0[1]) as u64).pow(2)} else {((c1.0[1] - c2.0[1]) as u64).pow(2)};
    let delta_b = if c2.0[2] > c1.0[2] {((c2.0[2] - c1.0[2]) as u64).pow(2)} else {((c1.0[2] - c2.0[2]) as u64).pow(2)};

    f64::sqrt((delta_r+delta_g+delta_b) as f64)
}

pub fn change_image_palette(palette: &Palette, image: DynamicImage)->Result<DynamicImage, std::sync::PoisonError<DynamicImage>>{
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
            let mut image= image_mutex.lock();
            image.as_deref_mut().expect("Image Mutex couldnt be dereferenced").put_pixel(x, y, modified_pixel);
            progress_bar.inc(1);
        });
    progress_bar.finish();
    let final_image = image_mutex.into_inner();
    match final_image {
        Ok(im) => Ok(im),
        Err(e) => Err(e)
    }
}

pub fn gruvbox_palette()->Palette{
    Palette{
    name: "gruvbox",
    colors: vec![
        Rgb([29, 32, 33]),
        Rgb([40, 40, 40]),
        Rgb([50, 48, 47]),
        Rgb([60, 56, 54]),
        Rgb([80, 73, 69]),
        Rgb([102, 92, 84]),
        Rgb([124, 111, 100]),
        Rgb([124, 111, 100]),
        Rgb([146, 131, 116]),
        Rgb([146, 131, 116]),
        Rgb([249, 245, 215]),
        Rgb([253, 244, 193]),
        Rgb([242, 229, 188]),
        Rgb([235, 219, 178]),
        Rgb([213, 196, 161]),
        Rgb([189, 174, 147]),
        Rgb([168, 153, 132]),
        Rgb([168, 153, 132]),
        Rgb([251, 73, 52]),
        Rgb([184, 187, 38]),
        Rgb([250, 189, 47]),
        Rgb([131, 165, 152]),
        Rgb([211, 134, 155]),
        Rgb([142, 192, 124]),
        Rgb([254, 128, 25]),
        Rgb([204, 36, 29]),
        Rgb([152, 151, 26]),
        Rgb([215, 153, 33]),
        Rgb([69, 133, 136]),
        Rgb([177, 98, 134]),
        Rgb([104, 157, 106]),
        Rgb([214, 93, 14]),
        Rgb([157, 0, 6]),
        Rgb([121, 116, 14]),
        Rgb([181, 118, 20]),
        Rgb([7, 102, 120]),
        Rgb([143, 63, 113]),
        Rgb([66, 123, 88]),
        Rgb([175, 58, 3]),
    ]}
}