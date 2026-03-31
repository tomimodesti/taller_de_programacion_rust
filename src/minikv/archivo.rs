//! Modulo archivo: maneja lo relacionado a busqueda en path,
//! apertura, lectura y escritura de archivos

use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::{
    fs::{File, OpenOptions},
    io::Write,
};

///Funcion que crea un archivo, si ya existe lo sobreescribe borrando su contenido
/// # Arguments
/// * `path` - Ruta del archivo a crear - &str
/// # Errores
/// * devuelve un error sino pudo abrir el archivo (no lo encontro o path incorrecta)
pub fn crear_archivo(path: &str) -> Result<File, String> {
    match std::fs::File::create(path) {
        Ok(file) => Ok(file),
        Err(e) => Err(format!("Error al crear el archivo {}: {}", path, e)),
    }
}

///Funcion que busca un archivo por un path dado
///     devuelve un Path al archivo si existe sino devuelve un error (no lo maneja)
/// # Argumentos
/// * `path` &str - path del archivo a abrir
/// # Errores
/// * devuelve un error sino se pudo enontrar el archivo ingresado como path
pub fn buscar_archivo(path: &str) -> Result<File, String> {
    let f = File::open(path); //devuelve un error o un file
    match f {
        //si lo encuentra devuelve el file, sino devuelve un error (no me interesa devolver un mensaje)
        Ok(file) => Ok(file),
        Err(_) => Err(format!("ERROR: {}", path)),
    }
}

///Funcion que abre un archivo para appendear, si no existe lo crea
/// # Arguments
/// * `path` - Ruta del archivo a abrir - &str
///
/// devuelve un File abierto para appendear o un mensaje de error si no se pudo abrir ni crear el archivo
///     devuelve el File o un mensaje de error
pub fn abrir_para_appendear(path: &str) -> Result<File, String> {
    match OpenOptions::new().append(true).create(true).open(path) {
        Ok(file) => Ok(file),
        Err(_) => match crear_archivo(path) {
            Ok(file) => Ok(file),
            Err(e) => Err(format!("{} {}", path, e)),
        },
    }
}

///Funcion que dado un archivo y un contenido (string) escribe en el archivo
/// #Arguments
/// * `file` - archivo a escribir
/// * `contenido` String - contenido a escribir en el archivo
pub fn escribir_archivo(mut file: File, contenido: String) -> Result<String, String> {
    match writeln!(file, "{}", contenido.trim()) {
        Ok(()) => {
            drop(file);
            Ok("OK".to_string())
        }
        Err(e) => Err(format!("{}", e)),
    }
}

///Funcion que dados el data y el log crea el hashmap de la base de datos hasta el momento
/// # Argumentos
/// * `data_path` &str - path al archivo data
/// * `log_path` &str - path al archivo log
/// # Errores
/// * al igual que cargar_hashMap, puede devolver errores de lectura de archivo
pub fn crear_hashmap(data_path: &str, log_path: &str) -> Result<HashMap<String, String>, String> {
    let mut hash_map: HashMap<String, String> = HashMap::new();
    //si los archivo aun no existen, queda igual el hashmap
    hash_map = cargar_hashmap_data(data_path, hash_map)?; //cargamos data
    hash_map = cargar_hashmap_log(log_path, hash_map)?; //cargamos log
    Ok(hash_map)
}

///Funcion que toma el path al data y guarda la informacion pertintente
///     respetando la estructura que deberia de tener el mismo:
///     `<key>` `<value>`
/// # Argumentos
/// * data_path - path al data
/// * hashmap - hashmap
/// # Errores
/// * Errores de estructura de log, si el data no tiene la estructura marcada aborta el programa
/// * Errores de lectura de archivos
fn cargar_hashmap_data(
    data_path: &str,
    mut hashmap: HashMap<String, String>,
) -> Result<HashMap<String, String>, String> {
    let archivo = buscar_archivo(data_path);
    let archivo_abierto = match archivo {
        Ok(file) => file,
        Err(_) => {
            return Ok(hashmap);
        }
    };
    let reader = BufReader::new(archivo_abierto);
    for line in reader.lines() {
        let linea = match line {
            Ok(l) => l,
            Err(_) => "INVALID DATA FILE".to_string(),
        };

        let linea = linea.trim();
        if linea.is_empty() {
            continue;
        }

        let partes: Vec<String> = Vec::new();
        match partes.as_slice() {
            [k, v] => {
                hashmap.insert(k.to_string(), v.to_string());
            }
            _ => return Err("INVALID DATA FILE".to_string()),
        }
    }
    Ok(hashmap)
}

///Funcion que toma el path al log y guarda la informacion pertintente
///     respetando la estructura que deberia de tener el mismo
///     "set" `<key>` `<value>` o "set" `<key>`
/// # Argumentos
/// * log_path - path al log
/// * hashmap - hashmap vacio
/// # Errores
/// * Errores de estructura de log, si el log no tiene la estructura marcada aborta el programa
/// * Errores de lectura de archivos
fn cargar_hashmap_log(
    log_path: &str,
    mut hashmap: HashMap<String, String>,
) -> Result<HashMap<String, String>, String> {
    let archivo = buscar_archivo(log_path);
    let archivo_abierto = match archivo {
        Ok(file) => file,
        Err(_) => {
            return Ok(hashmap);
        }
    };
    let reader = BufReader::new(archivo_abierto);
    for line in reader.lines() {
        let linea = match line {
            Ok(l) => l,
            Err(_) => "INVALID LOG FILE".to_string(),
        };
        let linea = linea.trim();
        if linea.is_empty() {
            continue;
        }
        let partes: Vec<String> = Vec::new();
        match partes.as_slice() {
            [op, k, v] if op == "set" => {
                hashmap.insert(k.to_string(), v.to_string());
            }
            [op, k] if op == "set" => {
                hashmap.remove(k);
            }
            _ => return Err("INVALID LOG FILE".to_string()),
        }
    }
    Ok(hashmap)
}

///Funcion usada para escribir en el data_file la informacion de la minikv,
///     quedandose solo con los pares `<clave>` `<valor>` que no fueron borrados
pub fn escrbir_data(mut data_file: File, hash_map: HashMap<String, String>) -> Result<(), String> {
    for (clave, valor) in &hash_map {
        let linea = format!("\"{}\" \"{}\"\n", clave, valor);
        data_file
            .write_all(linea.as_bytes())
            .map_err(|_| "Error al escribir".to_string())?;
    }
    Ok(())
}
