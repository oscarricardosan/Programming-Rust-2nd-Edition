# Introduction 

En este repositorio almacenare el código que se vaya creando en el curso  O'REILLY® - Programming Rust Fast, Safe Systems Development.
Este curso me permitira mejorar mi manejo de la programación en paralelo a la par de mis habilidades con Rust.

Ya que algunos ejemplos de cóigo del curso no son compatibles con versiones actuales se encontraran diferencias entre mi código y el del libro.

# Ejecución de contenedor

```
docker run --network host -it -v .:/app -w /app rust /bin/bash
cd closures
cargo run --bin trait
```