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

///Funcion que dado un path y un hashmap,
/// carga el hashmap con los datos del archivo,
/// si el archivo no existe devuelve el hashmap dado sin modificar
/// # Argumentos
/// * `path` - recibe un path al archivo que queremos leer para cargar el hashmap
/// * `hashmap` - recibe un hashmap ya inicializado
/// # Errores
/// * errores de lectura del archivo al abrir o leer
fn cargar_hashmap(path: &str, mut hashmap: HashMap<String, String>) -> HashMap<String, String> {
    let archivo = buscar_archivo(path);
    //asi la funcion me sirve para leer tanto data como log
    let archivo_abierto = match archivo {
        Ok(file) => file,
        Err(_) => {
            return hashmap;
        }
    };
    let reader = BufReader::new(archivo_abierto);

    for line in reader.lines() {
        let linea = match line {
            Ok(l) => l,
            Err(_) => continue,
        };
        let partes: Vec<&str> = linea.split_whitespace().collect();
        match partes.as_slice() {
            ["set", clave, valor] => {
                //si en el log hay un set, lo agrego al hashmap, si ya hay una clave igual se sobreescribe el valor
                //para evitar duplicados
                hashmap.insert(clave.to_string(), valor.to_string());
            }
            ["set", clave] => {
                //si en el log hay un unset
                hashmap.remove(*clave);
            }
            [k, v] => {
                hashmap.insert(k.to_string(), v.to_string());
            }
            _ => continue,
        }
    }
    hashmap
}

///Funcion que dados el data y el log crea el hashmap de la base de datos hasta el momento
/// # Argumentos
/// * `data_path` &str - path al archivo data
/// * `log_path` &str - path al archivo log
/// # Errores
/// * al igual que cargar_hashMap, puede devolver errores de lectura de archivo
pub fn crear_hashmap(data_path: &str, log_path: &str) -> HashMap<String, String> {
    let mut hash_map: HashMap<String, String> = HashMap::new();
    //si los archivo aun no existen, queda igual el hashmap
    hash_map = cargar_hashmap(data_path, hash_map); //cargamos data
    hash_map = cargar_hashmap(log_path, hash_map); //cargamos log
    hash_map
}
