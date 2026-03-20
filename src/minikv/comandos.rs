use std::collections::HashMap;

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

//TODO: implementar ejecutar_comando 
impl Comando {
    pub fn ejecutar(&self,hash_map : HashMap<String, String>) -> Result<String, String> {
        match self {
            Comando::Set { clave, valor } => ejecutar_set(clave,valor,hash_map),
            Comando::Get { clave } => ejecutar_get(clave,hash_map),
            Comando::Delete { clave } => ejecutar_delete(clave,hash_map),
            Comando::Length => ejecutar_length(hash_map),
            Comando::Snapshot => ejecutar_snapshot(hash_map),
        }
    }
}


//TODO: implementar funciones, quedo mas simple ahora con el hashmap cargado
// y las verificaciones se hacen mas simplen con este
fn ejecutar_set(clave:&String, valor:&String, hashmap: HashMap<String, String>) -> Result<String,String> {
    //testeo de parseo de comando, 
    Ok(format!("Set: clave = {:?}, valor = {:?}", clave, valor))
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
        Ok(format!("OK"))
    } else {
        Err(format!("NOT FOUND"))
    }
}

fn ejecutar_length(hash_map: HashMap<String, String>) -> Result<String,String> {
    //solo necesita el largo del "la base de datos" en este contexto (LOG + DATA) osea el hashmap
    Ok(format!("{}", hash_map.len()))
    //tampoco genera escrita en LOG
}  
fn ejecutar_snapshot(hash_map: HashMap<String, String>) -> Result<String,String> {
    //testeo
    Ok(format!("Snapshot"))
}
