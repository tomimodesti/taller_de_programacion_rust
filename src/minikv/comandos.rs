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
    pub fn imprimir_comando(&self) -> String {
        match self {
            Comando::Set { clave, valor } => format!("Comando Set: clave = {:?}, valor = {:?}", clave, valor),
            Comando::Get { clave } => format!("Comando Get: clave = {:?}", clave),
            Comando::Delete { clave } => format!("Comando Delete: clave = {:?}", clave),
            Comando::Length => "Comando Length".to_string(),
            Comando::Snapshot => "Comando Snapshot".to_string(),
        }
    }
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
    //testeo
    Ok(format!("Get: clave = {:?}", clave))
}
fn ejecutar_delete(clave:&String, hash_map: HashMap<String, String>) -> 
Result<String,String> {
    //testeo
    Ok(format!("Delete: clave = {:?}", clave))
}

fn ejecutar_length(hash_map: HashMap<String, String>) -> Result<String,String> {
    //testeo
    Ok(format!("Length"))
}
fn ejecutar_snapshot(hash_map: HashMap<String, String>) -> Result<String,String> {
    //testeo
    Ok(format!("Snapshot"))
}
