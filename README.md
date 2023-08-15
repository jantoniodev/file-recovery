# File Recovery

![Portada](https://miro.medium.com/v2/resize:fit:3840/format:webp/1*EGPIBqzbgv_9KkvIxdFUew.jpeg)

Este proyecto fue realizado con fines educativos con el objetivo de aprender a desarrollar un recuperador de archivos. Puedes ver todo el detalle de la implementación en mi blog. [blog.joseantonio.cl](https://blog.joseantonio.cl/desarrollando-un-recuperador-de-archivos-borrados-b3256d25f409)

Esta herramienta sirve para recuperar archivos borrados dentro de una unidad de disco. Por ahora solo se ha implementado de manera sencilla la búsqueda de archivos PNG la cual carece de muchas funcionalidades pero que sirve para entender los principios básicos de una herramienta de este tipo.

## Uso

Clona el proyecto en tu sistema local.
```bash
git clone https://github.com/jantoniodev/file-recovery.git
```

Crea una carpeta para almacenar los resultados encontrados.
```bash
mkdir resultados
```

Ejecuta el programa de ejemplo.
```bash
cargo run -- --filepath data.zip --output resultados
```

Tambien puedes ejecutarlo en una unidad de disco externa con permisos de administrador.
```bash
sudo cargo run -- --filepath /dev/disk4 --output resultados
```

## Archivos de ejemplo
Para probar la búsqueda localmente sin dependender de una unidad de disco puedes utilizar el zip de ejemplo *data.zip*. El contenido de este archivo se encuentra en la carpeta *data* y deberías encontrar 7 imágenes en formato png. Puedes agregar más archivos a la carpeta data y comprimir los archivos sin compresión con el siguiente comando:

```bash
zip -0 -r data.zip data/
```

## Contribuciones
Si deseas agregar un formato distinto o mejorar los que ya existen con gusto recibiré tu pull request.