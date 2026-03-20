


//forma comando: minikv "comando" "argumentos"
//el option me permite devolver un mensaje de error o un mensaje de resultado valido, dependiendo del comando y los argumentos
pub fn parseo_comando(args: Vec<String>) -> Result<Comando,String> {
    if args.len() < 2 {
        return Err("Error: comando no especificado".to_string());
    }
    // SET = set = Set , no distinguimos solo en el comando
    let mut iter = args.into_iter();
    iter.next(); //salteo el primer elemento que es el nombre del programa
    
    match iter.next() {
        Some(nombre_comando) => {
            let argumentos: Vec<String> = iter.collect();
            return decidir_comando(nombre_comando, argumentos);
        },
        None => return Err("Error: comando no especificado".to_string()),
    }
}

///Funcion que dado un comando y sus argumentos,
/// devuelve el comando correspondiente o 
/// un mensaje de error si el comando no es valido
/// # Arguments
/// ```
/// * `comando` - Comando a ejecutar - String
/// * `argumentos` - Argumentos del comando - Vec<String> 
/// ``````
/// Ejemplo: minikv set key value -> comando = set, argumentos = [key, value]
/// Ejemplo: minikv get key -> comando = get, argumentos = [key]
pub fn decidir_comando(comando:String, argumentos:Vec<String>) -> Result<Comando,String> {
    let mut valores = argumentos.into_iter();
    match comando{
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
