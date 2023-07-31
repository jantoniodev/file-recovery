/*
    Paso a paso.

    1. Leer un archivo ✅
    2. Leer un archivo binario ✅
    3. Encontrar una secuencia de bytes ✅
    4. Leer completamente un archivo PNG ✅
    5. Guardar la imagen encontrada en un archivo ✅
    6. Refactorizar y ordenar código ✅
    7. Generalizar tipos de archivo a encontrar
    8. Mejorar parámetros de consola y logs ✅

    -- Para crear el zip sin compresión:
        > zip -0 -r data.zip data/
*/

use clap::Parser;
use recover::recover_png;

mod formats;
mod recover;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(short, long)]
    filepath: String,

    #[arg(short, long)]
    output: String,
}

fn main() {
    let args = Args::parse();

    let output = args.output;
    let filepath = args.filepath;

    match recover_png(&filepath, &output) {
        Ok(count) => {
            println!("✅ Found {:?} files.", count);
        }
        Err(error) => {
            print!("❌ ERROR: {}", error);
        }
    }

}