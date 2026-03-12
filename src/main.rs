/// Documentacion:
// Comentario:

//importo solo el modulo contar_frecuencias del archivo guia1.rs
mod guia1;
use guia1::contar_frecuencias;

fn main() {
    let ruta = "archivos_ejemplo/ejemplo.txt";
    let ruta2 = "src/guia1.rs";
    let frecuencias = contar_frecuencias(ruta2);
    for (palabra, frecuencia) in frecuencias.iter() {
        println!("{}: {}", palabra, frecuencia);
    }
}

