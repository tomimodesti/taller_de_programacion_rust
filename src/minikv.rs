//nombre del LOG: .minikv.log --> guarda la "sesion"
//nombre del DATA: .minikv.data --> guarda la base de datos actual (estado actual de la base de datos)

/// Enum comandos disponibles:
/// Set: setea un valor para una clave, si la clave ya existe se sobreescribe el valor
/// Get: obtiene el valor de una clave, si la clave no existe devuelve un mensaje de
/// Delete: elimina una clave, si la clave no existe devuelve un mensaje de error
/// Length: devuelve la cantidad de claves almacenadas en la base de datos (no cuenta el log)
/// Snapshot: guarda el estado actual de la base de datos en el archivo DATA y vacia el log (archivo LOG)
enum Comando {
    Set{clave: Option<String>, valor: Option<String>}, 
    Get{clave: Option<String>}, 
    Delete{clave: Option<String>}, 
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
}
// cargo run --release -- set key value
// que le llega al programa: ["minikv", "set", "key", "value"]
/*comandos : minikv set <key> <value>
            minikv get <key>
            minikv delete <key>
            minikv length
            minikv snapshot
*/


// tomamos el comando y verificamos cual es y si es valido,
//el resto de valores se guardan en valores y se pasan al comando correspondiente
// ejemplo: minikv set key value -> comando = set, valores = [key, value]
//se devuelve un OK o un mensaje de error por cada comando (simple para este tp, se puede mejorar con un enum de resultados)

//idea: el resultado option es si (mensaje error, mensaje resultado valido)
//de la datatenemos el path, como del log 


//main: se recibe el comando y por ahora lo imprimimos, luego se parsea el comando y se ejecuta el comando correspondiente
pub fn main_tp() -> () {
    //recibimos el comando y lo parseamos
    let args: Vec<String> = std::env::args().collect();
    println!("Comando recibido: {:?}", args);
    //parseo
    let resultado = parseo_comando(args);
    println!("Resultado del parseo: {:?}", resultado);
}

//forma comando: minikv "comando" "argumentos"
//el option me permite devolver un mensaje de error o un mensaje de resultado valido, dependiendo del comando y los argumentos
fn parseo_comando(args: Vec<String>) -> Option<String> {
    if args.len() < 2 {
        return Some("Error: comando no especificado".to_string());
    }
    // SET = set = Set , no distinguimos solo en el comando
    let tag_comando = args[1].to_ascii_lowercase().to_string();
    let valores = args[2..].to_vec(); //puede tener algo o vacio
    let resultado = decidir_comando(tag_comando, valores);
    match resultado{
        Ok(_) => {
            //ejecuto el comando y devuelvo el resultado
            return Some(format!("Comando ejecutado: {}", &resultado.unwrap().imprimir_comando()));
        }
        Err(error) => {
            return Some(error);
        }
    }
}

fn decidir_comando(comando:String, argumentos:Vec<String>) -> Result<Comando,String> {
    let mut valores = argumentos.into_iter();
    match comando.as_str() {
        "set" => {
            if valores.len() < 1 || valores.len() > 2 {
                return Err("Error: comando set invalido,
                uso: minikv set <key> <value> o minikv set <key>".to_string());
            }else if valores.len() == 1 {
                return Ok(Comando::Delete { clave: valores.next() });
            }else {
                return Ok(Comando::Set { clave: valores.next(), valor: valores.next() });
            }
        }
        "get" => {
            if valores.len() != 1 {
                return Err("Error: comando get invalido, uso: minikv get <key>".to_string());
            }
                return Ok(Comando::Get { clave: valores.next() });
        }
        "length" => {
            if valores.len() != 0 {
                return Err("Error: comando length invalido, uso: minikv length".to_string());
            }
                return Ok(Comando::Length);
        }
        "snapshot" => {
            if valores.len() != 0 {
                return Err("Error: comando snapshot invalido, uso: minikv snapshot".to_string());
            }
                return Ok(Comando::Snapshot);
        }
        _ => {
            return Err("Error: comando no reconocido".to_string());
        }
    }
}