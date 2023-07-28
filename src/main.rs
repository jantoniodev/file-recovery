/*
    Paso a paso.

    1. Leer un archivo ✅
    2. Leer un archivo binario
    3. Encontrar una secuencia de bytes
    4. Leer completamente un archivo PNG
    5. Guardar la imagen encontrada en un archivo
    6. Refactorizar y ordenar código
    7. Generalizar tipos de archivo a encontrar
    8. Mejorar parámetros de consola
*/
use std::io::{self, Read};
use std::fs::File;

fn main() -> io::Result<()> {
    let mut file = File::open("./Cargo.toml")?;
    let mut buffer: [u8; 1] = [0];

    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        print!("{}", buffer[..n][0] as u8 as char);
    }

    Ok(())
}
