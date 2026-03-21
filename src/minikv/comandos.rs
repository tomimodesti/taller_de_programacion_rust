//! Modulo dedicado a definicion y manejo de los distintos tipos de comandos
//! permitidos para  manejar minikv, como SET, GET, SNAPSHOT
use crate::minikv::archivo::{abrir_para_appendear, crear_archivo, escribir_archivo};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

const LOG_PATH: &str = ".minikv.log";
const DATA_PATH: &str = ".minikv.data";

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


fn ejecutar_set(clave: &String, valor: &String) -> Result<String, String> {
    let log_line = format!("set {} {}", clave, valor);
    let log_file: File = match abrir_para_appendear(LOG_PATH) {
        Ok(file) => file,
        Err(_) => return Err("Error al abrir el LOG".to_string()),
    };
    match escribir_archivo(log_file, log_line) {
        Ok(_) => Ok("OK".to_string()),
        Err(_) => Err("Error al escribir el set en log".to_string()),
    }
}
fn ejecutar_get(clave: &String, hash_map: HashMap<String, String>) -> Result<String, String> {
    match hash_map.get(clave) {
        Some(valor) => Ok(format!("{:?}", valor)),
        None => Err("NOT FOUND".to_string()),
    }
}
fn ejecutar_delete(clave: &String, hash_map: HashMap<String, String>) -> Result<String, String> {
    if hash_map.contains_key(clave) {
        let log_line: String = format!("set {}", clave);
        let log_file: File = match abrir_para_appendear(LOG_PATH) {
            Ok(file) => file,
            Err(e) => {
                return Err(format!(
                    "Error al abrir el archivo de log {}: {}",
                    LOG_PATH, e
                ));
            }
        };
        match escribir_archivo(log_file, log_line) {
            Ok(_) => Ok("OK".to_string()),
            Err(e) => Err(format!(
                "Error al escribir en el archivo de log {}: {}",
                LOG_PATH, e
            )),
        }
    } else {
        Err("NOT FOUND".to_string())
    }
}

fn ejecutar_length(hash_map: HashMap<String, String>) -> Result<String, String> {
    Ok(format!("{}", hash_map.len()))
}
fn ejecutar_snapshot(hash_map: HashMap<String, String>) -> Result<String, String> {
    crear_archivo(LOG_PATH)?;
    let data = crear_archivo(DATA_PATH)?;
    escrbir_data(data, hash_map)?;
    Ok("Snapshot terminado".to_string())
}

fn escrbir_data(mut data_file: File, hash_map: HashMap<String, String>) -> Result<(), String> {
    for (clave, valor) in &hash_map {
        let linea = format!("{} {}\n", clave, valor);
        data_file
            .write_all(linea.as_bytes())
            .map_err(|_| "Error al escribir el DATA".to_string())?;
    }
    Ok(())
}
