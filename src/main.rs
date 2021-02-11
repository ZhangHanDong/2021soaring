
use image::{self, DynamicImage};
use std::str::from_utf8;
use std::path::Path;
use clap::{App, Arg, ArgMatches};
use anyhow;

fn handle_img(arg_matches: ArgMatches) -> DynamicImage {
    let img_name = arg_matches.value_of("INPUT").expect("Need INPUT");
    let img = match image::open(&Path::new(img_name)) {
        Ok(p) => p,
        Err(_) => panic!("Not a valid image path or could no open image"),
    };

    // resize if need
    // ./target/debug/img2ascii -s 196 86 ./images/rustlogo.jpeg
    let resized = match arg_matches.values_of_lossy("resize") {
        Some(v) => v.iter().map(|s| s.parse::<u32>().unwrap()).collect(),
        None => vec![80u32, 40u32],
    };

    img.resize_exact(resized[0], resized[1],  image::imageops::FilterType::Nearest)

}

fn output_to_ascii(value: &u8) -> &str {
    let ascii_chars  = [
        " ", ".", "^", ",", ":", "_", "=", "~", "+", "O", "o", "*",
        "#", "&", "%", "B", "@"
    ];
    
    let n_chars = ascii_chars.len() as u8;
    let step = 255u8 / n_chars;
    for i in 1..(n_chars - 1) {   
        let comp = &step * i;
        if value < &comp {
            let idx = (i - 1) as usize;
            return ascii_chars[idx]
        }
    }

    ascii_chars[ (n_chars - 1) as usize ]
}

fn print_to_terminal(img: DynamicImage) {
    let imgbuf = img.to_luma8();
    let ascii_art = imgbuf.pixels()
        .map( |p| output_to_ascii(&p.0[0]))
        .fold( String::new(), |s, p| s + p );

    ascii_art.as_bytes()
        .chunks(imgbuf.width() as usize)
        .map(from_utf8)
        .for_each(|s| println!("{}", s.unwrap()));
}

fn main() -> Result<(), anyhow::Error> {
    let arg_matches = App::new("img2ascii")
        .version("0.0.1")
        .author("blackanger")
        .about("Turn an image into ascii art!")
        .args( &[
            Arg::from_usage("<INPUT> 'Input the image file'"),
            Arg::from_usage("[resize] -s, --resize [width] [height] "),
        ])
        .get_matches();

    let img = handle_img(arg_matches);
    print_to_terminal(img);
    Ok(())
}
