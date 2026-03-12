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
fn ahoracado(palabra: String, intentos: u32) -> () {

    let mut salir = false; //sale por ganar o perder
    let mut intentos_restantes = intentos;
    let mut letras_adivinadas: Vec<char> = [];

    //while o loop, cualquiera es valido
    while !salir {
        println!("La palabra hasta el momento es: {}",mostrar_palabra(palabra, letras_adivinadas));
        println!("Intentos restantes{}",intentos_restantes);
        println!("Adivinaste las siguientes letras: {}",letras_adivinadas);
        println!("Ingresa una letra: ");
        let mut letra = recibir_letra();

        if palabra.contains(letra) {
            letras_adivinadas.push(letra);
        } else {
            intentos_restantes -= 1;
        }
    }
}

fn mostrar_palabra(palabra: String, letras_adivinadas: Vec<char>) -> String {
    let mut resultado = String::new();
    for letra in palabra.chars() {
        if letras_adivinadas.contains(&letra) {
            resultado.push(letra);
        } else {
            resultado.push('_');
        }
    }
    resultado
}

fn recibir_letra() -> char {
    io::stdin()
    .read_line(&mut v)
    .expect("Error leyendo la linea.");
    match v.trim() {
        //si es una letra
        v if v.is_alphabetic() && v.len() == 1 => v.chars().next().unwrap(),
        _ => {
            println!("Por favor ingresa una letra valida.");
            recibir_letra()
        } 
    }
}