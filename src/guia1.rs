/*Ejercicio 1: El objetivo del ejercicio es implementar un programa de consola para jugar al ahorcado.
 Bienvenido al ahorcado de FIUBA!
///
/// La palabra hasta el momento es: _ _ _ _ _ _
/// Adivinaste las siguientes letras:
/// Te quedan 5 intentos.
Ingresa una letra: r

La palabra hasta el momento es: _ _ _ _ _ r
Adivinaste las siguientes letras: r
Te quedan 5 intentos.
Ingresa una letra: c

Si se ingresa una letra que no forma parte de la palabra, se pierde un intento.

La lista de palabras se debe leer de un archivo de texto, donde cada línea del archivo contendrá una palabra. De esa lista, se deberá elegir una palabra (puede ser una selección secuencial de palabras).

El programa termina cuando se adivina correctamente la palabra pensada, o cuando se acabaron los intentos.
*/

//pensemos que recibe la palabra por consola
//en rust se_usa_snake_case
use std::io;

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