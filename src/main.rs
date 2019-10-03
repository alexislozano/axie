use clap::{crate_authors, crate_name, crate_version, App, Arg};
use std::fs;
use image;

mod character;
use character::Character;


fn process_image(path: &str, ratio: u16) -> String {
    match image::open(path) {
        Err(_) => {
            eprintln!("Cannot open image");
            "".to_string()
        },
        Ok(dynamic_image) => {
            println!("Image {} was correctly opened", path);
            let mut asciis = "".to_string();
            for pixel in dynamic_image.to_luma().pixels() {
                let character = Character::from_luma(pixel[0]);
                asciis.push_str(character.ascii());
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
            Arg::with_name("ratio")
                .short("r")
                .long("ratio")
                .value_name("RATIO")
                .help("Sets the number of character size to pixel ratio")
                .takes_value(true),
        )
        .get_matches();


    let input = matches.value_of("input").unwrap_or("input.png");
    let output = matches.value_of("output").unwrap_or("output.txt");
    let ratio = matches
        .value_of("ratio")
        .unwrap_or("10")
        .parse::<u16>()
        .unwrap();

    let result = process_image(&input, ratio);

    match fs::write(&output, &result) {
        Ok(_) => println!("Result was written in {}", output),
        Err(_) => eprintln!("Cannot write the output file"),
    }

}
