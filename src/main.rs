use image::ImageReader;
use std::{env, fmt::Error, path::PathBuf, process::exit};

mod palette;

fn main() -> Result<(), Error> {
    let palette_path = PathBuf::from(env::args().nth(1).expect(
        "No path given\nExecute program as 'cargo run -- path/to/palette.txt path/to/file.png'",
    ));
    let palette = palette::parse_palette(&palette_path).expect("Palette couldn't be parsed");

    let image_path = PathBuf::from(env::args().nth(2).expect("No image path given"));
    println!(
        "Loading Image {:?}",
        image_path
            .file_name()
            .expect("Image name couldn't be parsed")
    );
    let image_reader = ImageReader::open(&image_path).expect("Image couldn't be opened");
    let mut image = image_reader.decode().expect("Image couldn't be decoded");
    // let palette = palette::gruvbox_palette();

    println!("Modifying Image...");
    match palette::change_image_palette(&palette, image) {
        Ok(im) => image = im,
        Err(e) => {
            println!("Image couldn't be processed! \n{}", e);
            exit(1);
        }
    }

    println!("Saving Image...");
    // save image
    let parent_path = &image_path
        .parent()
        .expect("Parent path couldn't be resolved")
        .to_str()
        .expect("Parent path couldn't be cast to string");
    let file_name = &image_path
        .file_name()
        .expect("File name couldn't be resolved")
        .to_str()
        .expect("Filename couldn't be cast to string");
    let palette_name = palette_path
        .file_stem()
        .expect("Couldn't parse palette name")        
        .to_str()
        .expect("Palette name couldn't be cast to string");

    let new_path = format!("{}/{}_{}", parent_path, palette_name, file_name);

    image.save(&new_path).expect("Couldn't save image");
    println!("Image saved as \"{}\"", new_path);

    Ok(())
}
