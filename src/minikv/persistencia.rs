//! archivo para metodos de persistencia
use crate::minikv::archivo::truncar_archivo;
use crate::minikv::estructuras::MensajePersistencia;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex, RwLock};

pub fn manejar_snapshot(
    data: Arc<RwLock<File>>,
    log: Arc<Mutex<File>>,
    hashmap: Arc<RwLock<HashMap<String, String>>>,
) {
    let Ok(mut data_file) = data.write() else {
        println!("ERROR: lock de data en estado poisoned");
        return;
    };
    if let Err(e) = truncar_archivo(&mut data_file) {
        println!("{}", e.to_str());
        return;
    }
    let Ok(hashmap) = hashmap.read() else {
        println!("Lock de hashmap en estado poissoned");
        return;
    };
    for (clave, valor) in &*hashmap {
        //escribimos sobre data
        let line = format!("\"{}\" \"{}\"\n", clave, valor.replace("\"", "\\\""));
        match data_file.write_all(line.as_bytes()) {
            Ok(_) => {}
            Err(_) => {
                println!("ERROR: <no se pudo escribir en data>")
            }
        }
    }
    drop(data_file);
    let Ok(mut log_file) = log.lock() else {
        println!("LOG en estado poissoned");
        return;
    };
    if let Err(e) = truncar_archivo(&mut log_file) {
        println!("{}", e.to_str());
    }
}

pub fn manejar_delete(
    log: Arc<Mutex<File>>,
    hashmap: Arc<RwLock<HashMap<String, String>>>,
    clave: String,
) {
    let log_line: String = format!("set \"{}\"\n", clave);
    match log.lock() {
        Ok(mut log_file) => {
            //el lock al salir del scope se libera automaticamente
            match log_file.write_all(log_line.as_bytes()) {
                Ok(_) => {
                    println!("Operacion delete persistida en el log");
                }
                Err(e) => {
                    println!("ERROR: <Error al escribir en el log: {}>", e);
                }
            }
        }
        Err(_) => {
            println!("ERROR: <El lock de log esta poisoned>");
            return;
        }
    }
    match hashmap.write() {
        Ok(mut hashmap) => {
            hashmap.remove(&clave);
        }
        Err(_) => {
            println!("ERROR: <El lock de hashmap esta poisoned>");
        }
    }
}

pub fn manejar_set(
    log: Arc<Mutex<File>>,
    hashmap: Arc<RwLock<HashMap<String, String>>>,
    clave: String,
    valor: String,
) {
    let log_line: String = format!("set \"{}\" \"{}\"\n", clave, valor.replace("\"", "\\\""));
    match log.lock() {
        Ok(mut log_file) => {
            if let Err(e) = log_file.write_all(log_line.as_bytes()) {
                println!("ERROR: <Error al escribir en el log: {}>", e);
                return;
            }
            println!("Operacion set persistida en el log");
        }
        Err(_) => {
            println!("ERROR: <El lock de log esta poisoned>");
            return;
        }
    }
    match hashmap.write() {
        Ok(mut hashmap) => {
            hashmap.insert(clave, valor);
        }
        Err(_) => {
            println!("ERROR: <El lock de hashmap esta poisoned>");
        }
    }
}

pub fn manejar_persistencia(
    persistencia_receiver: Receiver<MensajePersistencia>,
    log: Arc<Mutex<File>>,
    data: Arc<RwLock<File>>,
    hashmap: Arc<RwLock<HashMap<String, String>>>,
) {
    //espera a que haya algo para escribir (set, delete o snapshot)
    for mensaje in persistencia_receiver {
        match mensaje {
            MensajePersistencia::Snapshot => {
                manejar_snapshot(data.clone(), log.clone(), hashmap.clone());
            }
            MensajePersistencia::Set { clave, valor } => {
                manejar_set(log.clone(), hashmap.clone(), clave, valor);
            }
            MensajePersistencia::Delete { clave } => {
                manejar_delete(log.clone(), hashmap.clone(), clave);
            }
        }
    }
}
