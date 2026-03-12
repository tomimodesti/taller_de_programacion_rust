/*
Codigo del tp1 de taller: 
    low-value stores (keys) - mini base de datos de clave-valor
    mini kvs persistente (persistencia a disco, en nuetsro caso un archivo)
    tendremos un LOG que guarde las operaciones 
    y un DATA que guarde los datos actuales (el estado actual de la base de datos)
    solo se gurdaran las operaciones tras usar snapshot

    Comandos disponibles para nuestra base de datos:
    set : minikv set <key> <value> 
    DELETE : minikv set <key> (value vacio)
    get : minikv get <key>
    lenght : minikv lenght
    snapshot : minikv snapshot
*/

/// Enum comandos disponibles:
/// Set: setea un valor para una clave, si la clave ya existe se sobreescribe el valor
/// Get: obtiene el valor de una clave, si la clave no existe devuelve un mensaje de
/// Delete: elimina una clave, si la clave no existe devuelve un mensaje de error
/// Length: devuelve la cantidad de claves almacenadas en la base de datos (no cuenta el log)
/// Snapshot: guarda el estado actual de la base de datos en el archivo DATA y vacia
enum Comando {
    Set(String, String), // key, value
    Get(String), //key
    Delete(String), // key
    Length,
    Snapshot,
}

// tomamos el comando y verificamos cual es y si es valido,
//el resto de valores se guardan en valores y se pasan al comando correspondiente
// ejemplo: minikv set key value -> comando = set, valores = [key, value]
//se devuelve un OK o un mensaje de error por cada comando (simple para este tp, se puede mejorar con un enum de resultados)
fn getComando(comando:String, valores: Vec<String>) -> String {
    

}