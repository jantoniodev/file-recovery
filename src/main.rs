/*
    Paso a paso.

    1. Leer un archivo ✅
    2. Leer un archivo binario ✅
    3. Encontrar una secuencia de bytes ✅
    4. Leer completamente un archivo PNG ✅
    5. Guardar la imagen encontrada en un archivo ✅
    6. Refactorizar y ordenar código ✅
    7. Generalizar tipos de archivo a encontrar
    8. Mejorar parámetros de consola

    -- Para crear el zip sin compresión:
        > zip -0 -r data.zip data/
*/

use recover::recover_png;

mod formats;
mod recover;

fn main() {
    let _ = recover_png("./data.zip", "./found");
}