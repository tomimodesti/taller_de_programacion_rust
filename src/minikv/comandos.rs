use std::{collections::HashMap};
use std::fs::File;
use crate::minikv::archivo::{abrir_para_appendear, escribir_archivo};

const LOG_PATH: &str = ".minikv.log";
const DATA_PATH: &str = ".minikv.data";

/// Enum comandos disponibles:
/// Set: setea un valor para una clave, si la clave ya existe se sobreescribe el valor
/// Get: obtiene el valor de una clave, si la clave no existe devuelve un mensaje de
/// Delete: elimina una clave, si la clave no existe devuelve un mensaje de error
/// Length: devuelve la cantidad de claves almacenadas en la base de datos (no cuenta el log)
/// Snapshot: guarda el estado actual de la base de datos en el archivo DATA y vacia el log (archivo LOG)
pub enum Comando {
    Set{clave: String, valor: String}, 
    Get{clave: String}, 
    Delete{clave: String}, 
    Length,
    Snapshot,
}

impl Comando {
    pub fn ejecutar(&self,hash_map : HashMap<String, String>) -> Result<String, String> {
        match self {
            Comando::Set { clave, valor } => ejecutar_set(clave,valor),
            Comando::Get { clave } => ejecutar_get(clave,hash_map),
            Comando::Delete { clave } => ejecutar_delete(clave,hash_map),
            Comando::Length => ejecutar_length(hash_map),
            Comando::Snapshot => ejecutar_snapshot(hash_map),
        }
    }
}


//TODO: implementar funciones, quedo mas simple ahora con el hashmap cargado
// y las verificaciones se hacen mas simplen con este
fn ejecutar_set(clave:&String, valor:&String) -> Result<String,String> {
    //que haria set: agregar en el log set clave valor,
    //despues si eso pisa alguna  existente seria una modificacion
    let log_line = format!("set {} {}",clave,valor);
    let log_file: File = match abrir_para_appendear(LOG_PATH) {
        Ok(file ) => file,
        Err(_) => return Err(format!("Error al abrir el LOG"))  
    };
    match escribir_archivo(log_file,log_line ) {
        Ok(_) => Ok(format!("OK")),
        Err(_) => Err(format!("Error al escribir el set en log"))
    }
}
fn ejecutar_get(clave:&String, hash_map: HashMap<String, String>) -> Result<String,String> {
    //get tiene 2 resultados, entontrar la key y devolver el valor o no encontrar la key y devolver un mensaje de error
    match hash_map.get(clave) {
        Some(valor) => Ok(format!("{:?}", valor)),
        None => Err(format!("NOT FOUND")),
    }
    //no genera escritura en LOG entonces solo devuelve el resultado y termina
}
fn ejecutar_delete(clave:&String, hash_map: HashMap<String, String>) -> 
Result<String,String> {
    //tiene 2 resultados, entontrar la key y eliminarla (escribir en el LOG) o no encontrar la key y devolver un mensaje de error
    if hash_map.contains_key(clave) {
        //escribir en el LOG "set {clave}"
        //buscamos el LOG, sino existe lo creamos, si hubo un error al crear sale un error y salimos al main
        let log_line: String = format!("set {}\n", clave);
        let log_file: File = match abrir_para_appendear(LOG_PATH) {
            Ok(file) => file,
            Err(e) => return Err(format!("Error al abrir el archivo de log {}: {}", LOG_PATH, e)),
        };
        match escribir_archivo(log_file, log_line) {
            Ok(_) => Ok(format!("OK")),
            Err(e) => return Err(format!("Error al escribir en el archivo de log {}: {}", LOG_PATH, e)),
        }

        
    } else {
        Err(format!("NOT FOUND"))
    }
}

fn ejecutar_length(hash_map: HashMap<String, String>) -> Result<String,String> {
    //solo necesita el largo del "la base de datos" en este contexto (LOG + DATA) osea el hashmap
    Ok(format!("{}", hash_map.len()))
    //tampoco genera escrita en LOG
}  
fn ejecutar_snapshot(_hash_map: HashMap<String, String>) -> Result<String,String> {
    //que hace snapshot: toma el log si existe
    Ok(format!(".."))
}
