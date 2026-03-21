//! Modulo main, archivo principal para manejo del minikv.
//! Modulos usados: archivo, comando y parseo
//! Se utilizo herramientas de la std, para los modulos (Files, HashMap,etc.)
mod minikv;
use minikv::archivo::crear_hashmap;
use minikv::comandos::Comando;
use minikv::parseo::parseo_comando;

const DATA_PATH: &str = ".minikv.data";
const LOG_PATH: &str = ".minikv.log";

///Funcion que dada los parametros inciales, modifica el LOG y el DATA de manera correspondiente
/// # Ejemplos
/// ```
/// Modos de uso: minikv set <key> <value> --> asigna key a value y lo escribe en log
///               minikv get <key> --> devuelve el value asignado a esa key si existe
/// ```
///  # Errores
/// Los errores son manejados, pero pueden surgir por mal inputs (comandos invalidos)
/// O errores de manejo de archivo, como al abrir o escribir
pub fn main() {
    let hash_map = crear_hashmap(DATA_PATH, LOG_PATH);
    let args: Vec<String> = std::env::args().collect();
    let comando: Result<Comando, String> = parseo_comando(args);
    match comando {
        
        Ok(comando_valido) => {
            let resultado = comando_valido.ejecutar(hash_map);
            match resultado {
                Ok(mensaje_resultado) => println!("{}", mensaje_resultado),
                Err(mensaje_error) => println!("{}", mensaje_error),
            }
        }
        Err(mensaje_error) => println!("{}", mensaje_error),
    }
}

//Tests:
