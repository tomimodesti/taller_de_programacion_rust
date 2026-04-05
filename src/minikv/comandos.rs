//! Modulo dedicado a definicion y manejo de los distintos tipos de comandos
//! permitidos para  manejar minikv, como SET, GET, SNAPSHOT
use std::sync::Arc;
use std::{collections::HashMap, sync::{RwLock, mpsc::Sender}};
use crate::minikv::estructuras::MensajePersistencia;

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
    pub fn ejecutar(&self, hashmap_lock: Arc<RwLock<HashMap<String,String>>>,sender:Sender<MensajePersistencia>) -> Result<String, String> {
        match self {
            Comando::Set { clave, valor } => ejecutar_set(clave, valor, sender,hashmap_lock),
            Comando::Get { clave } => ejecutar_get(clave, hashmap_lock),
            Comando::Delete { clave } => ejecutar_delete(clave, hashmap_lock,sender),
            Comando::Length => ejecutar_length(hashmap_lock),
            Comando::Snapshot => ejecutar_snapshot(hashmap_lock,sender),
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
fn ejecutar_set(_clave: &String, _valor: &str, _sender: Sender<MensajePersistencia>, _hashmap_lock: Arc<RwLock<HashMap<String, String>>>) -> Result<String, String> {
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
fn ejecutar_get(clave: &String, hashmap_lock: Arc<RwLock<HashMap<String, String>>>) -> Result<String, String> {
    //que haria get:
    //1) pide el lock de lectura del hashmap
    //2) busca la clave en el hashmap 
    //3) si la encontro devolvemos el valor sino NOT FOUND
    //4) se libera el lock cuando sale del scope de la funcion
    if let Ok(hashmap) = hashmap_lock.read() {
        if let Some(valor) = hashmap.get(clave) {
            return Ok(valor.to_string());
        }
        return Err("NOT FOUND".to_string());
    }
    //si llegamos aca el lock esta en estado poisoned
        Err("Error al obtener el lock de lectura".to_string())
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
fn ejecutar_delete(_clave: &String, _hashmap_lock: Arc<RwLock<HashMap<String, String>>>, _sender: Sender<MensajePersistencia>) -> Result<String, String> {
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
fn ejecutar_length(hashmap_lock: Arc<RwLock<HashMap<String, String>>>) -> Result<String, String> {
    if let Ok(hashmap) = hashmap_lock.read() {
        return Ok(hashmap.len().to_string());
    }
    //si llegamos aca el lock esta en estado poisoned
    Err("Error al obtener el lock de lectura".to_string())
}

///Funcion SNAPSHOT, funcion que toma tanto el data como el log
///     y unifica a ambos para quedarse con la informacion en data
///     truncando a log
/// # Ejemplo
/// * minikv snapshot
fn ejecutar_snapshot(_hashmap_lock: Arc<RwLock<HashMap<String, String>>>, sender: Sender<MensajePersistencia>) -> Result<String, String> {
    //que haria el snapshot: 
    //envia por el sender al thread de persistencia un mensaje "SNAPSHOT"
    //indicandole que debe realizar el snapshot
    match sender.send(MensajePersistencia::Snapshot) {
        Ok(_) => Ok("OK".to_string()),
        Err(e) => Err(format!("Error al enviar el mensaje de snapshot: {}", e)),
    }
}
