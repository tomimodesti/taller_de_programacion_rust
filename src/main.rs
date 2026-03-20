use std::io::{BufRead, BufReader};
use std::collections::HashMap;

mod minikv;
use minikv::parseo::{parseo_comando};
use minikv::archivo::{buscar_archivo};
use minikv::comandos::{Comando};

const DATA_PATH: &str = ".minikv.data";
const LOG_PATH: &str = ".minikv.log";

// cargo run --release -- set key value
// que le llega al programa: ["minikv", "set", "key", "value"]

//Estructura de DATA: los lenght y los snapshot no se guardan en el data, el log solo guarda los set y delete, el get no se guarda en ningun lado
// key1 value1
// key2 value2

//Estructura de LOG: 
// los lenght y los snapshot no se guardan en el log, solo los set y delete
// solo guarda comandos de escritura, no de lectura
// set key value
// set key (si saltea el valor es un delete)

// tomamos el comando y verificamos cual es y si es valido,
//el resto de valores se guardan en valores y se pasan al comando correspondiente
// ejemplo: minikv set key value -> comando = set, valores = [key, value]
//se devuelve un OK o un mensaje de error por cada comando (simple para este tp, se puede mejorar con un enum de resultados)

//idea: el resultado option es si (mensaje error, mensaje resultado valido)
//de la datatenemos el path, como del log 


//main: se recibe el comando y por ahora lo imprimimos, luego se parsea el comando y se ejecuta el comando correspondiente
pub fn main() -> () {
    let mut hash_map: HashMap<String, String> = HashMap::new();
    //si los archivo aun no existen, queda igual el hashmap
    hash_map = cargar_hashmap(DATA_PATH, hash_map); //cargamos data
    hash_map = cargar_hashmap(LOG_PATH, hash_map); //cargamos log
    //recibimos el comando y lo parseamos
    let args: Vec<String> = std::env::args().collect();
    //parseo
    let comando: Result<Comando,String> = parseo_comando(args);
    //que me gustaria: porque cada comando lanza su propio mensaje de error o resultado, entonces el resultado del parseo es un comando o un mensaje de error,
    // y luego el resultado de ejecutar el comando es un mensaje de resultado o un mensaje de error, entonces deberia tener dos match anidados, uno para el parseo y otro para la ejecucion del comando
    match comando { //del parseo del comando
        Ok(comando_valido) => {
            let resultado = comando_valido.ejecutar(hash_map);
            match resultado { //de la ejecuccion del comando
                Ok(mensaje_resultado) => println!("{}", mensaje_resultado),
                Err(mensaje_error) => println!("{}", mensaje_error),
            }
        },
        Err(mensaje_error) => println!("{}", mensaje_error),
        }
}



///Funcion que dado un path y un hashmap, 
/// carga el hashmap con los datos del archivo, 
/// si el archivo no existe devuelve el hashmap dado sin modificar
fn cargar_hashmap(path: &str, mut hashmap: HashMap<String, String>) -> HashMap<String, String> {

    let archivo = buscar_archivo(path);
    //asi la funcion me sirve para leer tanto data como log
    let archivo_abierto = match archivo {
        Ok(file) => file,
        Err(_) => return hashmap,
    };
    let reader = BufReader::new(archivo_abierto);

    for line in reader.lines() {
        if let Ok(linea) = line {
            let partes:Vec<&str> = linea.split_whitespace().collect();
            match partes.as_slice() {
                ["set", clave, valor] => { //si en el log hay un set, lo agrego al hashmap, si ya hay una clave igual se sobreescribe el valor
                                            //para evitar duplicados
                    hashmap.insert(clave.to_string(), valor.to_string());
                },
                ["set", clave] => { //si en el log hay un unset
                    hashmap.remove(*clave);
                },
                [k,v] => {
                    hashmap.insert(k.to_string(), v.to_string());
                }
                _ => continue, //si el data o log esta "corrupto" no es importante por el momento
            }
        }
    }
    hashmap
}

