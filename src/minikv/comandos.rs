//! Modulo dedicado a definicion y manejo de los distintos tipos de comandos
//! permitidos para  manejar minikv, como SET, GET, SNAPSHOT

use std::collections::HashMap;

/// Enum comandos disponibles:
/// Set: setea un valor para una clave, si la clave ya existe se sobreescribe el valor
/// Get: obtiene el valor de una clave, si la clave no existe devuelve un mensaje de
/// Delete: elimina una clave, si la clave no existe devuelve un mensaje de error
/// Length: devuelve la cantidad de claves almacenadas en la base de datos (no cuenta el log)
/// Snapshot: guarda el estado actual de la base de datos en el archivo DATA y vacia el log (archivo LOG)
pub enum Comando {
    Set { clave: String, valor: String },
    Get { clave: String },
    Delete { clave: String },
    Length,
    Snapshot,
}

///metodo compartido por todos los comandos
///  para conseguir cierto polimorfismo sin objetos
impl Comando {
    pub fn ejecutar(&self, hash_map: HashMap<String, String>) -> Result<String, String> {
        match self {
            Comando::Set { clave, valor } => ejecutar_set(clave, valor),
            Comando::Get { clave } => ejecutar_get(clave, hash_map),
            Comando::Delete { clave } => ejecutar_delete(clave, hash_map),
            Comando::Length => ejecutar_length(hash_map),
            Comando::Snapshot => ejecutar_snapshot(hash_map),
        }
    }
}

///Funcion SET, toma la `<clave>` `<valor>` ingresados y los agrega a la base de datos
///     si habia una clave igual, pisa el valor anterior.
/// #Argumentos:
/// * `clave` &String - clave que identifica al valor
/// * `valor` &string - valor indexado por la clave
/// # Ejemplos
/// ```text
/// * minikv set index valor
/// index sera la clave del valor "valor"
/// ```
/// # Errores
/// * errores para abrir los archivos correspondientes
/// * errores para escribir en los archivos
/// # Devuelve
/// * OK --> si pudo ingresar el conjunto clave valor
/// * mensaje de error sino pudo terminar la operacion
fn ejecutar_set(_clave: &String, _valor: &str) -> Result<String, String> {
    /*let log_line = format!("set \"{}\" \"{}\"", clave, valor.replace("\"", "\\\""));
    let log_file: File = match abrir_para_appendear(LOG_PATH) {
        Ok(file) => file,
        Err(_) => return Err("Error al abrir el LOG".to_string()),
    };
    match escribir_archivo(log_file, log_line) {
        Ok(_) => Ok("OK".to_string()),
        Err(_) => Err("Error al escribir el set en log".to_string()),
    }*/
    Ok("Ok".to_string())
}

/// Funcion GET, dada una clave, busca en la base si hay un valor asignado a la misma
///     si no existe lo indica por terminal
/// #Argumentos:
/// * `clave` &String - clave que identifica al valor
/// * `hash_map` HashMap <String,String> - hashmap de la base
/// # Ejemplo:
/// ```text
/// * minikv get index
///   ------------- (terminal)
///     "valor" o "NOT FOUND"
/// ```
/// # Errores
/// * errores para abrir los archivos correspondientes
/// * errores para leer en los archivos
/// # Devuelve
/// * el valor que fue asignada a esa clave
/// * NOT FOUND --> si no pudo encontrar el valor de esa clave
fn ejecutar_get(_clave: &String, _hash_map: HashMap<String, String>) -> Result<String, String> {
    /*match hash_map.get(clave) {
        Some(valor) => {
            let valor_limpio = valor;
            Ok(valor_limpio.to_string())
        }
        None => Err("NOT FOUND".to_string()),
    }*/
    Ok("Ok".to_string())
}

///Funcion DELETE, toma la clave ingresada y
///      trata de borrar el valor asignado a la misma si existe
/// # Argumentos
/// * `clave` &String - clave que identifica al valor
/// * `hash_map` HashMap <String,String> - hashmap de la base
///   #Ejemplo
/// ```text
/// * minikv set clave
/// ```
///   # Errores
/// * errores para abrir los archivos correspondientes
fn ejecutar_delete(_clave: &String, _hash_map: HashMap<String, String>) -> Result<String, String> {
    /*let log_line: String = format!("set \"{}\"", clave);
    let log_file: File = match abrir_para_appendear(LOG_PATH) {
        Ok(file) => file,
        Err(_) => {
            return Err("INVALID LOG FILE".to_string());
        }
    };
    match escribir_archivo(log_file, log_line) {
        Ok(_) => Ok("OK".to_string()),
        Err(e) => Err(format!(
            "Error al escribir en el archivo de log {}: {}",
            LOG_PATH, e
        )),
    }
    */
    Ok("Ok".to_string())
}

///Funcion LENGTH, devuelve el largo de la base hasta el momento
/// #Ejemplo
/// * minikv length
///   -------------- (terminal)
///   3
fn ejecutar_length(_hash_map: HashMap<String, String>) -> Result<String, String> {
    Ok("Ok".to_string())
}

///Funcion SNAPSHOT, funcion que toma tanto el data como el log
///     y unifica a ambos para quedarse con la informacion en data
///     truncando a log
/// # Ejemplo
/// * minikv snapshot
fn ejecutar_snapshot(_hash_map: HashMap<String, String>) -> Result<String, String> {
    Ok("Ok".to_string())
}
