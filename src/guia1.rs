
//pensemos que recibe la palabra por consola
//en rust se_usa_snake_case
use std::io;

#[allow(dead_code)]
pub fn ahoracado(palabra: String, intentos: u32) -> () {

    let mut intentos_restantes = intentos;
    let mut letras_ingresadas: Vec<char> = Vec::new();

    //while o loop, cualquiera es valido
    while intentos_restantes > 0 {
        let actual: String = mostrar_palabra(&palabra, &letras_ingresadas);
        println!("La palabra hasta el momento es: {}", actual);
        println!("Intentos restantes: {}", intentos_restantes);

        let letra = recibir_letra();

        if letras_ingresadas.contains(&letra) {
            println!("Ya habías intentado con la letra '{}'", letra);
            continue;
        }

        letras_ingresadas.push(letra);

        if !palabra.contains(letra) {
            intentos_restantes -= 1;
            println!("La letra '{}' no está.", letra);
        }
        // Verificamos si ganó comparando sin espacios
        let despues = mostrar_palabra(&palabra, &letras_ingresadas);
        if !despues.contains('_') {
            println!("¡Ganaste! La palabra era: {}", palabra);
            return;
        }
    }
    println!("¡Perdiste! La palabra era: {}", palabra);
    
}

fn mostrar_palabra(palabra: &str, letras_adivinadas: &Vec<char>) -> String {
    palabra.chars()
        .map(|c| if letras_adivinadas.contains(&c) { c } else { '_' })
        .collect::<Vec<_>>()
        .iter()
        .map(|c| format!("{} ", c)) // Estética: "a _ _ o "
        .collect()
}

fn recibir_letra() -> char {
    let mut v: String = String::new();
    print!("Ingresa una letra: ");
    io::stdin()
    .read_line(&mut v)
    .expect("Error leyendo la linea.");

    let trimmed = v.trim();

    match trimmed.chars().next() {
        //si es una letra
        Some(trimmed) if trimmed.is_alphabetic()=>trimmed.to_ascii_lowercase(),
        // si no se que es
        _ => {
            println!("Por favor ingresa una letra valida.");
            recibir_letra()
        } 
    }
}

//ejercicio 2:
/* 
Escribir un programa para contar las frecuencias de palabras únicas leídas de un archivo de entrada. Luego imprimirlas con sus frecuencias, ordenadas primero por las más frecuentes. Por ejemplo, dado este archivo de entrada:

La casa tiene una ventana
La ventana fue defenestrada

El programa debe imprimir:

ventana -> 2
La -> 2
casa -> 1
tiene -> 1
una -> 1
fue -> 1
defenestrada -> 1

Una solución básica consiste en leer el archivo línea por línea, convertirlo a minúsculas, dividir cada línea en palabras y contar las frecuencias en un HashMap. Una vez hecho esto, convertir el HashMap en una lista de pares de palabras y cantidad y ordenarlas por cantidad (el más grande primero) y por último imprimirlos.

Se debe seguir las siguientes recomendaciones:

    Para separar en palabras, se debe considerar los espacios en blanco, ignorando los signos de puntuación.
    Si la frecuencia de dos palabras es la misma, no importa el orden en el que aparecen las dos palabras en la salida impresa.
    No leer el archivo completo en memoria, se puede ir procesando línea por línea, o en conjuntos de líneas. Sí se puede mantener en memoria el hashmap completo.
    Usar solamente las herramientas de la biblioteca std del lenguaje.

Para leer un archivo línea por línea, se puede utilizar el método read_line.
*/
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

pub fn contar_frecuencias(archivo: &str) -> HashMap<String,u32> {

    let contenido = abrir_archivo(archivo);
    let mut frecuencias: HashMap<String,u32> = HashMap::new();

    //si no pudo abrir el archivo, devuelvo un string con el error
    //si contenido es un error sale por el if let
    if let Err(_e) = contenido {
        return HashMap::from([("Error".to_string(), 0)]); // Devolvemos un HashMap con un mensaje de error
    }
    //si llegamos el archivo se abri correctamente,
    //entonces leemos linea por linea, agregando los elementos al hashmap
    let file = File::open(archivo).unwrap();
    //buffer de lectura para leer el archivo linea por linea
    let lector = BufReader::new(file);

    for linea in lector.lines(){
        match linea {
            Ok(linea) => {
                //procesamos la linea en palabras
                let palabras = procesar_linea(&linea);
                //actualizo el hashmap con las palabras obtenidas
                modificar_frecuencias(&mut frecuencias, palabras);
            }
            Err(e) => {
                println!("Error leyendo la linea: {}", e);

            }
        }
    }
    return frecuencias;
}

//funcion para abrir el archivo, devuelve un resultado, si es exitoso devuelve un unit, sino devuelve un error
fn abrir_archivo(archivo: &str) -> std::io::Result<()> {
    File::open(archivo)?;
    Ok(())
}

fn procesar_linea(linea:&str) ->Vec<String> {
    linea.split_whitespace()
        .map(|s| quitar_signos(s))
        .map(|s| s.to_lowercase())
        .collect()
}

fn quitar_signos(palabra: &str) -> String {
    palabra.chars()
        .filter(|c| c.is_alphanumeric())
        .collect()
}

fn modificar_frecuencias(frecuencias: &mut HashMap<String,u32>,palabras:Vec<String>) {
    for palabra in palabras{
        //busca la palabra en el hash, sino esta la crea con valor 0, y le suma 1
        *frecuencias.entry(palabra).or_insert(0) += 1;
    }
}
