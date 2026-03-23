//! Modulo main, archivo principal para manejo del minikv.
//! Modulos usados: archivo, comando y parseo
//! Se utilizo herramientas de la std, para los modulos (Files, HashMap,etc.)
mod minikv;
use minikv::archivo::crear_hashmap;
use minikv::comandos::Comando;
use minikv::parseo::parseo_comando;

const DATA_PATH: &str = ".minikv.data";
const LOG_PATH: &str = ".minikv.log";

///Funcion que dada los parametros inciales, modifica el LOG y el DATA de manera correspondiente
/// # Ejemplos
/// ```
/// Modos de uso: minikv set <key> <value> --> asigna key a value y lo escribe en log
///               minikv get <key> --> devuelve el value asignado a esa key si existe
///               minikv set <key> --> desasigna key al valor que tenga si esta presente
///               minikv length --> devuelve la cantidad de pares clave valor
///               minikv snapshot --> unfica data y log guardando todos los pares clave valor validos en data.
/// ```
///  # Errores
/// Los errores son manejados, pero pueden surgir por mal inputs (comandos invalidos)
/// O errores de manejo de archivo, como al abrir o escribir
pub fn main() {
    let hash_map = crear_hashmap(DATA_PATH, LOG_PATH);
    let args: Vec<String> = std::env::args().collect();
    let comando: Result<Comando, String> = parseo_comando(args);
    match comando {
        Ok(comando_valido) => {
            let resultado = comando_valido.ejecutar(hash_map);
            match resultado {
                Ok(mensaje_resultado) => println!("{}", mensaje_resultado),
                Err(mensaje_error) => println!("ERROR: {}", mensaje_error),
            }
        }
        Err(mensaje_error) => println!("ERROR: {}", mensaje_error),
    }
}

//Tests:

#[cfg(test)]
mod tests {

    use super::*;
    use std::thread;
    use std::time::Duration;
    use std::{
        fs,
        process::{Command, Stdio},
    };

    fn limpiar_archivos() {
        let _ = fs::remove_file(".minikv.data");
        let _ = fs::remove_file(".minikv.log");
    }

    #[test]
    fn test_pareso_valido() {
        let argumentos = vec![
            "minikv".to_string(),
            "set".to_string(),
            "nombre".to_string(),
            "messi".to_string(),
        ];
        let resultado = parseo_comando(argumentos);
        assert!(resultado.is_ok());
    }

    #[test]
    fn test_parseo_invalido() {
        //comando invalido
        let argumentos = vec!["minikv".to_string(), "length".to_string(), "a".to_string()];
        let resultado = parseo_comando(argumentos);
        assert!(!resultado.is_ok());
    }

    #[test]
    fn test_ejecuccion_set() {
        limpiar_archivos();
        let child = Command::new("target/debug/minikv")
            .arg("set")
            .arg("clave")
            .arg("valor")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Fallo al ejecutar");
        let output = child.wait_with_output().expect("Fallo al esperar");
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("OK"));
        limpiar_archivos();
    }

    #[test]
    fn test_ejecuccion_get() {
        limpiar_archivos();
        let _child = Command::new("target/debug/minikv")
            .arg("set")
            .arg("clave")
            .arg("valor")
            .output()
            .expect("Fallo al ejecutar");
        thread::sleep(Duration::from_millis(200));
        let get = Command::new("target/debug/minikv")
            .arg("get")
            .arg("clave")
            .stdout(Stdio::piped())
            .spawn()
            .expect("Error al ejecutar");
        let get_output = get.wait_with_output().expect("Fallo al esperar");
        let get_stdout = String::from_utf8_lossy(&get_output.stdout);
        assert!(get_stdout.contains("valor"));
        limpiar_archivos();
    }

    #[test]
    fn test_ejecuccion_lenght() {
        limpiar_archivos();

        let comando1 = Command::new("target/debug/minikv")
            .args(["set", "clave", "valor"])
            .status()
            .expect("Fallo al ejecutar");
        assert!(comando1.success(), "el primer set no termino correctamente");
        let comando2 = Command::new("target/debug/minikv")
            .args(["set", "a", "b"])
            .status()
            .expect("Fallo al ejecutar");
        assert!(
            comando2.success(),
            "el segundo comando no termino correctamente"
        );
        let comando3 = Command::new("target/debug/minikv")
            .args(["set", "c", "d"])
            .status()
            .expect("Fallo al ejecutar");
        assert!(
            comando3.success(),
            "el tercer comando no termino correctamente"
        );
        let child = Command::new("target/debug/minikv")
            .arg("length")
            .output()
            .expect("Fallo al iniciar");
        let stdout = String::from_utf8_lossy(&child.stdout);
        let stdout = stdout.trim();
        assert_eq!(stdout, "3");
        limpiar_archivos();
    }

    #[test]
    fn test_consistencia_snapshot() {
        limpiar_archivos();
        let comando_set = Command::new("target/debug/minikv")
            .args(["set", "clave", "valor"])
            .status()
            .expect("Fallo al ejecutar");
        assert!(
            comando_set.success(),
            "el comando set no termino correctamente"
        );
        //get pre snapshot
        let comando_get = Command::new("target/debug/minikv")
            .args(["get", "clave"])
            .output()
            .expect("Error al ejecutar get");
        let salida_get = String::from_utf8_lossy(&comando_get.stdout);
        let salida_get = salida_get.trim();
        let comando_snapshot = Command::new("target/debug/minikv")
            .args(["snapshot"])
            .status()
            .expect("Error al ejecutar snapshot");
        assert!(
            comando_snapshot.success(),
            "El comando snapshot no se realizo correctamente"
        );
        //get post snapshot
        let comando_get_2 = Command::new("target/debug/minikv")
            .args(["get", "clave"])
            .output()
            .expect("Error al ejecutar get");
        let salida_get_2 = String::from_utf8_lossy(&comando_get_2.stdout);
        let salida_get_2 = salida_get_2.trim();
        assert_eq!(salida_get, salida_get_2)
    }
}
