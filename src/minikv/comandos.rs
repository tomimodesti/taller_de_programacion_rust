//! Modulo dedicado a definicion y manejo de los distintos tipos de comandos
//! permitidos para  manejar minikv, como SET, GET, SNAPSHOT
use crate::minikv::estructuras::MensajePersistencia;
use std::sync::Arc;
use std::{
    collections::HashMap,
    sync::{RwLock, mpsc::Sender},
};

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
    pub fn ejecutar(
        self,
        hashmap_lock: Arc<RwLock<HashMap<String, String>>>,
        sender: Sender<MensajePersistencia>,
    ) -> Result<String, String> {
        match self {
            Comando::Set { clave, valor } => ejecutar_set(clave, valor, sender),
            Comando::Get { clave } => ejecutar_get(clave, hashmap_lock),
            Comando::Delete { clave } => ejecutar_delete(clave, sender),
            Comando::Length => ejecutar_length(hashmap_lock),
            Comando::Snapshot => ejecutar_snapshot(sender),
        }
    }
}

///Funcion SET, toma la `<clave>` `<valor>` ingresados y los agrega a la base de datos
///     si habia una clave igual, pisa el valor anterior.
/// #Argumentos:
/// * `clave` String - clave que identifica al valor
/// * `valor` String - valor indexado por la clave
/// # Ejemplos
/// ```text
/// * > minikv set index arob
/// index sera la clave del valor "arbol"
/// ```
/// # Errores
/// * Posible error al obtener el lock de escritura del hashmap (estado poisoned)
/// * Posible error al enviar el mensaje de set al thread de persistencia
/// # Devuelve
/// * OK --> si pudo ingresar el conjunto clave valor
/// * mensaje de error sino pudo terminar la operacion
fn ejecutar_set(
    clave: String,
    valor: String,
    sender: Sender<MensajePersistencia>,
) -> Result<String, String> {
    let mensaje = MensajePersistencia::Set { clave, valor };
    match sender.send(mensaje) {
        Ok(_) => Ok("OK".to_string()),
        Err(e) => Err(format!("Error al enviar el mensaje de set: {}", e)),
    }
}

/// Funcion GET, dada una clave, busca en la base si hay un valor asignado a la misma
///     si no existe lo indica por terminal
/// #Argumentos:
/// * `clave` String - clave que identifica al valor
/// * `hash_map` HashMap <String,String> - hashmap de la base
/// # Ejemplo:
/// ```text
/// * minikv get index
///   ------------- (terminal)
///     "valor" o "NOT FOUND"
/// ```
/// # Errores
/// * Posible error al obtener el lock de lectura del hashmap (estado poisoned)
/// * Posible error al buscar la clave en el hashmap
/// # Devuelve
/// * el valor que fue asignada a esa clave
/// * NOT FOUND --> si no pudo encontrar el valor de esa clave
fn ejecutar_get(
    clave: String,
    hashmap_lock: Arc<RwLock<HashMap<String, String>>>,
) -> Result<String, String> {
    //que haria get:
    //1) pide el lock de lectura del hashmap
    //2) busca la clave en el hashmap
    //3) si la encontro devolvemos el valor sino NOT FOUND
    //4) se libera el lock cuando sale del scope de la funcion
    if let Ok(hashmap) = hashmap_lock.read() {
        if let Some(valor) = hashmap.get(&clave) {
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
/// * `clave` String - clave que identifica al valor
/// * `hash_map` HashMap <String,String> - hashmap de la base
///   #Ejemplo
/// ```text
/// * minikv set clave
/// ```
///   # Errores
/// * Posible error al obtener el lock de escritura del hashmap (estado poisoned)
/// * Posible error al eliminar la clave del hashmap
/// * Posible error al enviar el mensaje de delete al thread de persistencia
/// # Devuelve
/// * OK --> si pudo eliminar la clave (si no existia no se hace nada pero se devuelve OK)
/// * mensaje de error sino pudo terminar la operacion
fn ejecutar_delete(clave: String, sender: Sender<MensajePersistencia>) -> Result<String, String> {
    let mensaje = MensajePersistencia::Delete { clave };
    match sender.send(mensaje) {
        Ok(_) => Ok("OK".to_string()),
        Err(e) => Err(format!("Error al enviar el mensaje de delete: {}", e)),
    }
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
fn ejecutar_snapshot(sender: Sender<MensajePersistencia>) -> Result<String, String> {
    //que haria el snapshot:
    //envia por el sender al thread de persistencia un mensaje "SNAPSHOT"
    //indicandole que debe realizar el snapshot
    match sender.send(MensajePersistencia::Snapshot) {
        Ok(_) => Ok("OK".to_string()),
        Err(e) => Err(format!("Error al enviar el mensaje de snapshot: {}", e)),
    }
}
