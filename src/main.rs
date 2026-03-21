mod minikv;
use minikv::archivo::crear_hashmap;
use minikv::comandos::Comando;
use minikv::parseo::parseo_comando;

const DATA_PATH: &str = ".minikv.data";
const LOG_PATH: &str = ".minikv.log";

//TODO: Documentar funciones y fmt

///Funcion que dada los parametros inciales, modifica el LOG y el DATA de manera correspondiente
/// ```
/// Modos de uso: minikv set <key> <value> --> asigna key a value y lo escribe en log
///               minikv get <key> --> devuelve el value asignado a esa key si existe
/// ```
pub fn main() {
    let hash_map = crear_hashmap(DATA_PATH, LOG_PATH);
    //recibimos el comando y lo parseamos
    let args: Vec<String> = std::env::args().collect();
    //parseo
    let comando: Result<Comando, String> = parseo_comando(args);
    //que me gustaria: porque cada comando lanza su propio mensaje de error o resultado, entonces el resultado del parseo es un comando o un mensaje de error,
    // y luego el resultado de ejecutar el comando es un mensaje de resultado o un mensaje de error, entonces deberia tener dos match anidados, uno para el parseo y otro para la ejecucion del comando
    match comando {
        //del parseo del comando
        Ok(comando_valido) => {
            let resultado = comando_valido.ejecutar(hash_map);
            match resultado {
                //de la ejecuccion del comando
                Ok(mensaje_resultado) => println!("{}", mensaje_resultado),
                Err(mensaje_error) => println!("{}", mensaje_error),
            }
        }
        Err(mensaje_error) => println!("{}", mensaje_error),
    }
}

//Tests:
