//! Modulo archivo: maneja lo relacionado a busqueda en path,
//! apertura, lectura y escritura de archivos

use crate::minikv::errores::KvErrores;
use crate::minikv::estructuras::Storage;
use crate::minikv::parseo::procesar_linea;
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

///Funcion que abre un archivo pasandole un path como argumento e indicando si
/// el modo de escritura es append o escritura basica
pub fn abrir_archivo(path: &str, append: bool) -> Result<File, KvErrores> {
    //abrir el archivo, si no existe lo crea
    //si append es true, se abre para agregar al final, sino se sobreescribe
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(append)
        .open(path);
    match file {
        Ok(f) => Ok(f),
        Err(_) => Err(KvErrores::Error("ERROR al abrir archivo".to_string())),
    }
}

///Funcion que dados el data y el log crea el hashmap de la base de datos hasta el momento
/// # Argumentos
/// * `data_path` Box<dyn Storage> -  data
/// * `log_path`  Box<dyn Storage> -  log
/// # Errores
/// * al igual que cargar_hashMap, puede devolver errores de lectura de archivo
pub fn cargar_hashmap(
    data_path: &mut dyn Storage,
    log_path: &mut dyn Storage,
) -> Result<HashMap<String, String>, KvErrores> {
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
/// * data_path - algo que funciona como Storage (podemos leer y escribir)
/// * hashmap - hashmap
/// # Errores
/// * Errores de estructura de log, si el data no tiene la estructura marcada aborta el programa
/// * Errores de lectura de archivos
fn cargar_hashmap_data(
    data_path: &mut dyn Storage,
    mut hashmap: HashMap<String, String>,
) -> Result<HashMap<String, String>, KvErrores> {
    let reader = BufReader::new(data_path);
    for line in reader.lines() {
        let linea = match line {
            Ok(l) => l,
            Err(_) => return Err(KvErrores::InvalidDataFile),
        };

        let linea = linea.trim();
        if linea.is_empty() {
            continue;
        }

        let partes: Vec<String> = procesar_linea(linea);
        match partes.as_slice() {
            [k, v] => {
                hashmap.insert(k.to_string(), v.to_string());
            }
            _ => return Err(KvErrores::InvalidDataFile),
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
    log_path: &mut dyn Storage,
    mut hashmap: HashMap<String, String>,
) -> Result<HashMap<String, String>, KvErrores> {
    let reader = BufReader::new(log_path);
    for line in reader.lines() {
        let linea = match line {
            Ok(l) => l,
            Err(_) => return Err(KvErrores::InvalidLogFile),
        };
        let linea = linea.trim();
        if linea.is_empty() {
            continue;
        }
        let partes: Vec<String> = procesar_linea(linea);
        match partes.as_slice() {
            [op, k, v] if op == "set" => {
                hashmap.insert(k.to_string(), v.to_string());
            }
            [op, k] if op == "set" => {
                hashmap.remove(k);
            }
            _ => return Err(KvErrores::InvalidLogFile),
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

///Funcion para abrir data y log con sus respectivos modos de escritura
/// puede devolver errores de apertura de archivos, caso contrario devuelve
/// ambos archivos
pub fn abrir_archivos(data_path: &str, log_path: &str) -> Result<(File, File), KvErrores> {
    let data_file = abrir_archivo(data_path, false)?;
    let log_file = abrir_archivo(log_path, true)?;
    Ok((data_file, log_file))
}
