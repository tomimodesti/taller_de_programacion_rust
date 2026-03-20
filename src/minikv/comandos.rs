/// Enum comandos disponibles:
/// Set: setea un valor para una clave, si la clave ya existe se sobreescribe el valor
/// Get: obtiene el valor de una clave, si la clave no existe devuelve un mensaje de
/// Delete: elimina una clave, si la clave no existe devuelve un mensaje de error
/// Length: devuelve la cantidad de claves almacenadas en la base de datos (no cuenta el log)
/// Snapshot: guarda el estado actual de la base de datos en el archivo DATA y vacia el log (archivo LOG)
enum Comando {
    Set{clave: String, valor: String}, 
    Get{clave: String}, 
    Delete{clave: String}, 
    Length,
    Snapshot,
}

//TODO: implementar ejecutar_comando 
impl Comando {
    fn imprimir_comando(&self) -> String {
        match self {
            Comando::Set { clave, valor } => format!("Comando Set: clave = {:?}, valor = {:?}", clave, valor),
            Comando::Get { clave } => format!("Comando Get: clave = {:?}", clave),
            Comando::Delete { clave } => format!("Comando Delete: clave = {:?}", clave),
            Comando::Length => "Comando Length".to_string(),
            Comando::Snapshot => "Comando Snapshot".to_string(),
        }
    }
    fn ejecutar(&self) -> Result<String, String> {
        match self {
            Comando::Set { clave, valor } => ejecutar_set(clave,valor),
            Comando::Get { clave } => ejecutar_get(clave),
            Comando::Delete { clave } => ejecutar_delete(clave),
            Comando::Length => ejecutar_length(),
            Comando::Snapshot => ejecutar_snapshot(),
        }
    }
}


//TODO: implementar funciones, quedo mas simple ahora con el hashmap cargado
// y las verificaciones se hacen mas simplen con este
fn ejecutar_set(clave:String, valor:String, mut hashmap: HashMap<String, String>) -> Result<String> {

}
fn ejecutar_get(clave:String, hashmap: HashMap<String, String>) -> Result<String> {

}
fn ejecutar_delete(clave:String, mut hashmap: HashMap<String, String>) -> 
Result<String> {

}

fn ejecutar_length(hashmap: HashMap<String, String>) -> Result<String> {

}
fn ejecutar_snapshot(hashmap: HashMap<String, String>) -> Result<String> {

}
