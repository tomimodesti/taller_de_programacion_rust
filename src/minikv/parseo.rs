//! Modulo para parseo de comandos e inputs de usuario

use crate::minikv::{comandos::Comando, errores::KvErrores};

/// Funcion que dado el input del usuario,
///  lo parsea y decide que comando se ejecutara
/// # Argumentos
/// ```text
/// `args` `Vec<String>` - argumentos dados por el usuario a nuestra motor de almacenamiento
/// ```
pub fn parseo_comando(args: Vec<String>) -> Result<Comando, KvErrores> {
    if args.is_empty() {
        return Err(KvErrores::Error(
            "Error: comando no especificado".to_string(),
        ));
    }
    let mut iter = args.into_iter();

    match iter.next() {
        Some(mut nombre_comando) => {
            nombre_comando = nombre_comando.to_lowercase();
            let argumentos: Vec<String> = iter.collect();
            decidir_comando(&nombre_comando, argumentos)
        }
        None => Err(KvErrores::Error(
            "Error: comando no especificado".to_string(),
        )),
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
pub fn decidir_comando(comando: &str, argumentos: Vec<String>) -> Result<Comando, KvErrores> {
    let valores = argumentos.into_iter();
    match comando {
        "set" => procesar_set(valores),
        "get" => procesar_get(valores),
        "length" => procesar_length(valores),
        "snapshot" => procesar_snapshot(valores),
        _ => Err(KvErrores::UnknownCommand),
    }
}

fn procesar_set(mut args: impl Iterator<Item = String>) -> Result<Comando, KvErrores> {
    match (args.next(), args.next(), args.next()) {
        (Some(clave), None, None) => Ok(Comando::Delete { clave }),
        (Some(clave), Some(valor), None) => Ok(Comando::Set { clave, valor }),
        (None, _, _) => Err(KvErrores::MissingArgument),
        (_, _, Some(_)) => Err(KvErrores::ExtraArgument),
    }
}
fn procesar_get(mut args: impl Iterator<Item = String>) -> Result<Comando, KvErrores> {
    match (args.next(), args.next()) {
        (Some(clave), None) => Ok(Comando::Get { clave }),
        (None, _) => Err(KvErrores::MissingArgument),
        (_, Some(_)) => Err(KvErrores::ExtraArgument),
    }
}
fn procesar_length(mut args: impl Iterator<Item = String>) -> Result<Comando, KvErrores> {
    if args.next().is_none() {
        Ok(Comando::Length)
    } else {
        Err(KvErrores::ExtraArgument)
    }
}
fn procesar_snapshot(mut args: impl Iterator<Item = String>) -> Result<Comando, KvErrores> {
    if args.next().is_none() {
        Ok(Comando::Snapshot)
    } else {
        Err(KvErrores::ExtraArgument)
    }
}

///Funcion que dado una linea la procesa devolviendo un vector con sus partes
pub fn procesar_linea(linea: &str) -> Vec<String> {
    let mut partes = Vec::new();
    let mut actual = String::new();
    let mut en_comillas = false;
    let mut escapado = false;
    for c in linea.chars() {
        if escapado {
            actual.push(c);
            escapado = false;
        } else if c == '\\' && en_comillas {
            escapado = true;
        } else if c == '"' {
            en_comillas = !en_comillas;
        } else if c.is_whitespace() && !en_comillas {
            if !actual.is_empty() {
                partes.push(actual.clone());
                actual.clear();
            }
        } else {
            actual.push(c);
        }
    }
    if !actual.is_empty() {
        partes.push(actual);
    }
    partes
}
