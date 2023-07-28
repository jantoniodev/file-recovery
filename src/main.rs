/*
    Paso a paso.

    1. Leer un archivo ✅
    2. Leer un archivo binario ✅
    3. Encontrar una secuencia de bytes ✅
    4. Leer completamente un archivo PNG
    5. Guardar la imagen encontrada en un archivo
    6. Refactorizar y ordenar código
    7. Generalizar tipos de archivo a encontrar
    8. Mejorar parámetros de consola

    -- Para crear el zip sin compresión:
        > zip -0 -r data.zip data/
*/
use std::io::{self, Read};
use std::fs::File;

struct PNG {
    match_index: usize,
}

impl PNG {
    const MAGIC: [u8; 8] = [0x89,0x50,0x4E,0x47,0x0D,0x0A,0x1A,0x0A];

    fn new() -> PNG {
        PNG { match_index: 0 }
    }

    fn check_byte(&mut self, byte: u8) -> bool {
        if self.match_index > 7 {
            self.match_index = 0;
        }

        if Self::MAGIC[self.match_index] == byte {
            self.match_index += 1;
        } else {
            self.match_index = 0;
        }

        return self.match_index == 8;
    }
}

fn main() -> io::Result<()> {
    let mut file = File::open("./data.zip")?;
    let mut buffer = [0; 1];

    let mut png = PNG::new();

    let mut position = 0;
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        
        let found = png.check_byte(buffer[..n][0]);

        if found {
            println!("Found png on {}", position - PNG::MAGIC.len() + 1)
        }

        position += 1;
    }

    Ok(())
}
