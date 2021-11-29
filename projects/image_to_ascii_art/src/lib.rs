extern crate image;

use image::GenericImageView;

use std::error::Error;  //allows for some better errors
use std::fs;            //the library that will allow us to parse files
//use std::env;           //gives access to environment stuff

//the image struct should include the filename, file path, extension, and image data

//TESTs
#[cfg(test)]
mod tests {
    //use super::*;

    //add tests as i go along

}


//IMAGE code
pub struct Config {
    pub file_path: String,
    pub img: image::DynamicImage,
}
impl Config {
    /**
     * creates a new image from the passed args (taken from command line)
     */
    pub fn new(args: &[String]) -> Result<Config, &str> {
        // parse arguments
        if args.len() < 2 { // not enough arguments (will always be at least 2)
            return Err("not enough arguments");
        }
        let file_path = args[1].clone();//get file path from input
        let img = image::open(&file_path).unwrap(); // open image

        //return
        Ok(Config { file_path, img } )
    }

}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //call to another function to do the expensive stuff
    println!("{} as ascii art (resolution reduced to fit in terminal): \n\n", config.file_path);
    println!("{}",image_to_ascii(&config));

    //return
    Ok(())
}

pub fn image_to_ascii(config: &Config) -> String {
    //let mut txt_output: Vec<&str> = Vec::new();
    let printable_output: String;

    let img = config.img.grayscale();

    //generate ascii art of whole image
    fs::write("./ascii_art.txt", format!("{}",gen_ascii(img.clone().into_rgb8()))).expect("Couldn't write to file");

    //generate ascii art of resized image
    printable_output = String::from(gen_ascii(img.clone().resize_to_fill(100, 100*img.height()/img.width(), image::imageops::Nearest).into_rgb8()));

    return printable_output;
}
fn gen_ascii(img: image::RgbImage) -> String {
    let mut output: String = String::new();
    let pixel_luminance: Vec<f32>;
    const PALLET_SIZE:usize = 5;
    let ascii_characters: [char; PALLET_SIZE] = [' ','░','▒','▓','█']; //pretty much the pallet
    let mut luminance_scale: [f32;PALLET_SIZE] = [0.0;PALLET_SIZE];

    //calculate and store lumincance of every pixel here to save time
    pixel_luminance = img.pixels().map(|p| {
        let image::Rgb(data) = *p;
        let (r,g,b) = (data[0] as f32, data[1] as f32, data[2] as f32);
        ((0.299*r*r + 0.587*g*g + 0.114*b*b)).sqrt()
    }).collect();
    
    //find the brightest and darkest, to use to scale rest of image
    let mut min = *pixel_luminance.get(0).unwrap();
    let mut max = *pixel_luminance.get(0).unwrap();
    for l in pixel_luminance.iter() {
        if *l > max {max = *l;}
        else if *l < min {min = *l;}
    }

    //calculate luminance scale for image
    for i in 1..=PALLET_SIZE {
        luminance_scale[i-1] = (max-min)/(i) as f32;
    }

    //parse image once more, and add the character closest to the darkness of each pixel to output
    for (index, l) in pixel_luminance.iter().enumerate() {
        //find closest match
        let mut best_match = 0;
        for i in 0..PALLET_SIZE {
            if (*l-luminance_scale[best_match]).abs() > (*l-luminance_scale[i]).abs() {
                best_match = i;
            }
        }

        output.push(ascii_characters[best_match]);
        output.push(ascii_characters[best_match]);

        if (index + 1) % img.width()as usize==0 {output.push('\n');}
    }

    return output;
}