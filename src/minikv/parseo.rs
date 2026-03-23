//! Modulo para parseo de comandos e inputs de usuario

use crate::minikv::comandos::Comando;

/// Funcion que dado el input del usuario,
///  lo parsea y decide que comando se ejecutara
/// # Argumentos
/// ```text
/// `args` `Vec<String>` - argumentos dados por el usuario a nuestra motor de almacenamiento
/// ```
pub fn parseo_comando(args: Vec<String>) -> Result<Comando, String> {
    if args.len() < 2 {
        return Err("Error: comando no especificado".to_string());
    }
    let mut iter = args.into_iter();
    iter.next(); //salteo el primer elemento que es el nombre del programa

    match iter.next() {
        Some(nombre_comando) => {
            let argumentos: Vec<String> = iter.collect();
            decidir_comando(&nombre_comando, argumentos)
        }
        None => Err("Error: comando no especificado".to_string()),
    }
}

///Funcion que dado un comando y sus argumentos,
/// devuelve el comando correspondiente o
/// un mensaje de error si el comando no es valido
/// # Arguments
/// ```text
/// * `comando` - Comando a ejecutar - String
/// * `argumentos` - Argumentos del comando - Vec<String>
/// ```
/// Ejemplo: minikv set key value -> comando = set, argumentos = [key, value]
/// Ejemplo: minikv get key -> comando = get, argumentos = [ "key" ]
pub fn decidir_comando(comando: &str, argumentos: Vec<String>) -> Result<Comando, String> {
    let mut valores = argumentos.into_iter();
    match comando {
        "set" => {
            if valores.len() == 1 {
                let clave = match valores.next() {
                    Some(c) => c,
                    None => {
                        return Err("MISSING ARGUMENT".to_string());
                    }
                };
                Ok(Comando::Delete { clave })
            } else if valores.len() == 2 {
                let clave = match valores.next() {
                    Some(c) => c,
                    None => {
                        return Err("MISSING ARGUMENT".to_string());
                    }
                };
                let valor = match valores.next() {
                    Some(v) => v,
                    None => {
                        return Err("MISSING ARGUMENT".to_string());
                    }
                };
                Ok(Comando::Set { clave, valor })
            } else if valores.len() == 0 {
                Err("MISSING ARGUMENT".to_string())
            } else {
                Err("EXTRA ARGUMENT".to_string())
            }
        }
        "get" => {
            if valores.len() == 0 {
                return Err("MISSING ARGUMENT".to_string());
            } else if valores.len() > 1 {
                return Err("EXTRA ARGUMENT".to_string());
            }
            let clave = match valores.next() {
                Some(c) => c,
                None => {
                    return Err("MISSING ARGUMENT".to_string());
                }
            };
            Ok(Comando::Get { clave })
        }
        "length" => {
            if valores.len() != 0 {
                return Err("EXTRA ARGUMENT".to_string());
            }
            Ok(Comando::Length)
        }
        "snapshot" => {
            if valores.len() != 0 {
                return Err("EXTRA ARGUMENT".to_string());
            }
            Ok(Comando::Snapshot)
        }
        _ => Err("UNKNOWN COMMAND".to_string()),
    }
}
