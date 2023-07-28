/*
    Paso a paso.

    1. Leer un archivo ✅
    2. Leer un archivo binario ✅
    3. Encontrar una secuencia de bytes ✅
    4. Leer completamente un archivo PNG ✅
    5. Guardar la imagen encontrada en un archivo
    6. Refactorizar y ordenar código
    7. Generalizar tipos de archivo a encontrar
    8. Mejorar parámetros de consola

    -- Para crear el zip sin compresión:
        > zip -0 -r data.zip data/
*/
use std::io::{self, Read};
use std::fs::File;

enum State {
    SearchingInit,
    SearchingEnd,
}

struct PNG {
    position: usize,
    match_index: usize,
    png_bytes: Vec<u8>,
    state: State,
}

impl PNG {
    const MAGIC_INIT: [u8; 8] = [0x89,0x50,0x4E,0x47,0x0D,0x0A,0x1A,0x0A];
    const MAGIC_END: [u8; 4] = [0x49,0x45,0x4E,0x44];

    fn new() -> PNG {
        PNG {
            position: 0,
            match_index: 0,
            png_bytes: Vec::new(),
            state: State::SearchingInit
        }
    }

    fn step(&mut self, byte: u8) -> Option<Vec<u8>> {
        match self.state {
            State::SearchingInit => {
                let found = self.check_init(byte);
                if found {
                    println!("Found png on {}", self.position - PNG::MAGIC_INIT.len() + 1);
                    self.state = State::SearchingEnd;
                }
            }

            State::SearchingEnd => {
                self.png_bytes.push(byte);
                let found = self.check_end(byte);
                if found {
                    self.state = State::SearchingInit;
                    let mut complete_png_bytes: Vec<u8> = Vec::new();
                    complete_png_bytes.extend_from_slice(&Self::MAGIC_INIT);
                    complete_png_bytes.extend_from_slice(&self.png_bytes);

                    self.png_bytes = Vec::new();

                    return Some(complete_png_bytes);
                }
            }
        }

        self.position += 1;
        None
    }

    fn check_init(&mut self, byte: u8) -> bool {
        if self.match_index > PNG::MAGIC_INIT.len() - 1 {
            self.match_index = 0;
        }

        if Self::MAGIC_INIT[self.match_index] == byte {
            self.match_index += 1;
        } else {
            self.match_index = 0;
        }

        return self.match_index == PNG::MAGIC_INIT.len();
    }

    fn check_end(&mut self, byte: u8) -> bool {
        if self.match_index > PNG::MAGIC_END.len() - 1 {
            self.match_index = 0;
        }

        if Self::MAGIC_END[self.match_index] == byte {
            self.match_index += 1;
        } else {
            self.match_index = 0;
        }

        return self.match_index == PNG::MAGIC_END.len();
    }
}

fn main() -> io::Result<()> {
    let mut file = File::open("./data.zip")?;
    let mut buffer = [0; 1];

    let mut png = PNG::new();

    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }

        match png.step(buffer[..n][0]) {
            Some(image) => {
                println!("PNG: {:?} bytes", image.len());
            }
            None => {

            }
        }

    }

    Ok(())
}
