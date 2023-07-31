use crate::formats::png::PNG;

use std::io::{self, Read, Write};
use std::fs::File;

pub fn recover_png(filepath: &str, output: &str) -> io::Result<i32> {
    let mut file = File::open(filepath)?;
    let mut buffer = [0; 32];

    let mut png = PNG::new();
    let mut founds = 0;
    let mut byte: usize = 0;

    loop {
        let n: usize = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }

        for buffer_byte in buffer.iter_mut() {
            match png.step(*buffer_byte) {
                Some(image) => {
                    founds += 1;
                    let file_name = format!("image{}.png", founds);
                    println!("📁 PNG: {} {:?} bytes", file_name, image.len());
                    save_file(image, output, &file_name);
                }
                None => {
    
                }
            }
        }

        byte += n;
        print_progress(byte);
    }

    Ok(founds)
}

fn save_file(file_bytes: Vec<u8>, folder: &str, filename: &str) {
    let file_name = format!("{}/{}", folder, filename);
    let mut file = File::create(file_name).unwrap();
    let _ = file.write_all(&file_bytes);
}

fn print_progress(progress: usize) {
    let megabytes = progress / 1000000;

    print!("{:?} Mb\r", megabytes);
}