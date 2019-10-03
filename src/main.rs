use clap::{crate_authors, crate_name, crate_version, App, Arg};
use std::fs;
use image;

mod character;
use character::Character;


fn process_image(path: &str, width: u32) -> String {
    match image::open(path) {
        Err(_) => {
            eprintln!("Cannot open image");
            "".to_string()
        },
        Ok(dynamic_image) => {
            println!("Image {} was correctly opened", path);

            let gray_image = dynamic_image.to_luma();

            let width_pixels = gray_image.width();
            let height_pixels = gray_image.height();
            
            let pixels_per_characters = if width_pixels / width > 0 {
                width_pixels / width
            } else {
                1
            };
            let height = height_pixels / pixels_per_characters;
            
            let mut asciis = "".to_string();
            for y in 0..height {
                for x in 0..width {
                    let mut pixel: u32 = 0;
                    for j in 0..pixels_per_characters {
                        for i in 0..pixels_per_characters {
                            pixel += gray_image.get_pixel(
                                x * pixels_per_characters + i,
                                y * pixels_per_characters + j
                            )[0] as u32;
                        }
                    }
                    pixel /= pixels_per_characters.pow(2);
                    let character = Character::from_luma(pixel as u8);
                    asciis.push_str(character.ascii());
                }
                asciis.push_str("\n");
            }

            asciis
        }
    }
}


fn main() {

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about("An image to ASCII converter")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("INPUT")
                .help("Input file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("OUTPUT")
                .help("Output file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .value_name("WIDTH")
                .help("Sets the width in characters of the resulting text")
                .takes_value(true),
        )
        .get_matches();


    let input = matches.value_of("input").unwrap_or("input.png");
    let output = matches.value_of("output").unwrap_or("output.txt");
    let width = match matches.value_of("width").unwrap_or("50").parse::<u32>() {
        Ok(width) => match width {
            0 => 50,
            width => width,
        },
        Err(_) => 50
    };

    let result = process_image(&input, width);

    match fs::write(&output, &result) {
        Ok(_) => println!("Result was written in {}", output),
        Err(_) => eprintln!("Cannot write the output file"),
    }

}
