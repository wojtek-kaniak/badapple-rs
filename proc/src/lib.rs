use std::{time::SystemTime, io::BufReader, fs::File};

use image::{ImageFormat, DynamicImage};
use itertools::Itertools;
use proc_macro::TokenStream;
use static_assertions::const_assert;

extern crate proc_macro;

const FRAME_COUNT: usize = 6571;
const FRAMES_DIR: &str = "frames";
const FPS: usize = 60;
const FRAME_WIDTH: usize = 480;
#[allow(dead_code)]
const FRAME_HEIGHT: usize = 360;
const GRAYSCALE_MIDPOINT: u8 = 128;
// \u{1b}[2J
const BEGIN: &str = r#"compile_error!("\u{1b}[H\n\r"#;
const END: &str = r#"");"#;

#[proc_macro]
pub fn bad_apple(_input: TokenStream) -> TokenStream {
    let mut output = String::from(BEGIN);

    let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() * (FPS as u128) / 1000;
    let frame = (timestamp % FRAME_COUNT as u128) as usize;
    let image = load_frame(frame).unwrap();
    for (top, bottom) in image.to_luma8().chunks(FRAME_WIDTH).tuples() {
        const_assert!(FRAME_HEIGHT % 2 == 0);
        for ((top_left, top_right), (bottom_left, bottom_right)) in top.iter().tuples().zip(bottom.iter().tuples()) {
            let top_left = *top_left > GRAYSCALE_MIDPOINT;
            let top_right = *top_right > GRAYSCALE_MIDPOINT;
            let bottom_left = *bottom_left > GRAYSCALE_MIDPOINT;
            let bottom_right = *bottom_right > GRAYSCALE_MIDPOINT;

            output.push(match ((top_left, top_right), (bottom_left, bottom_right)) {
                ((false, false), (false, false)) => ' ',
                ((false, false), (false, true))  => '\u{2597}',
                ((false, false), (true,  false)) => '\u{2596}',
                ((false, false), (true,  true))  => '\u{2584}',
                ((false, true),  (false, false)) => '\u{259D}',
                ((false, true),  (false, true))  => '\u{2590}',
                ((false, true),  (true,  false)) => '\u{259E}',
                ((false, true),  (true,  true))  => '\u{259F}',
                ((true,  false), (false, false)) => '\u{2598}',
                ((true,  false), (false, true))  => '\u{259A}',
                ((true,  false), (true,  false)) => '\u{258C}',
                ((true,  false), (true,  true))  => '\u{2599}',
                ((true,  true),  (false, false)) => '\u{2580}',
                ((true,  true),  (false, true))  => '\u{259C}',
                ((true,  true),  (true,  false)) => '\u{259B}',
                ((true,  true),  (true,  true))  => '\u{2588}',
            });
            // output.push(if *pixel > GRAYSCALE_MIDPOINT {
            //     '#'
            // } else {
            //     ' '
            // });
        }
        output.push_str(r#"\n\r"#);
    }

    output.push_str(END);
    output.parse().unwrap()
}
// #[proc_macro]
// pub fn bad_apple(_input: TokenStream) -> TokenStream {
//     let mut text = String::from("                                ");
//     let offset = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis().div(250) as usize % 32;
//     text.replace_range(offset..offset, "#");
//     let mut output = String::new();
//     output.push_str(r#"compile_error!("\u{1b}[2J\u{1b}[H"#);
//     output.push_str(&text);
//     output.push_str(r#"\n\r"#);
//     output.push_str(&text);
//     output.push_str(r#"\n\r"#);
//     output.push_str(&text);
//     output.push_str(r#"\n\r"#);
//     output.push_str(&text);
//     output.push_str(r#"\n\r"#);
//     output.push_str(&text);
//     output.push_str(r#"\n\r"#);
//     output.push_str(&text);
//     output.push_str(r#"\n\n\n\n\n"#);
//     output.push_str(r#"\n\n\n\n\n"#);
//     output.push_str(r#"\n\n\n\n\n"#);
//     output.push_str(r#"\n\n\n\n\n"#);
//     output.push_str(r#"\n\n\n\n\n"#);
//     output.push_str(r#"\n\n\n\n\n"#);
//     output.push_str(r#"\n\n\n\n\n"#);
//     output.push_str("\");");
//     output.parse().unwrap()
// }

fn load_frame(ix: usize) -> Result<DynamicImage, ()> {
    let file = format!("{}/{}.jpg", FRAMES_DIR, ix);
    let file = File::open(file).map_err(|_| ())?;
    image::load(BufReader::new(file), ImageFormat::Jpeg).map_err(|_| ())
}