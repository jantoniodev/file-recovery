use crate::formats::png::PNG;

use std::io::{self, Read, Write};
use std::fs::File;

pub fn recover_png(filepath: &str, output: &str) -> io::Result<i32> {
    let mut file = File::open(filepath)?;
    let mut buffer = [0; 1];

    let mut png = PNG::new();
    let mut founds = 0;

    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }

        match png.step(buffer[..n][0]) {
            Some(image) => {
                founds += 1;
                println!("PNG: {:?} bytes", image.len());
                let file_name = format!("image{}.png", founds);
                save_file(image, output, &file_name);
            }
            None => {

            }
        }
    }

    Ok(founds)
}

fn save_file(file_bytes: Vec<u8>, folder: &str, filename: &str) {
    let file_name = format!("{}/{}", folder, filename);
    let mut file = File::create(file_name).unwrap();
    let _ = file.write_all(&file_bytes);
}