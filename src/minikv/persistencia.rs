//! archivo para metodos de persistencia
use std::sync::{Arc,Mutex,RwLock};
use std::fs::File;
use std::collections::HashMap;
use std::io::{Seek,SeekFrom,Write};


pub fn manejar_snapshot(
    data: Arc<RwLock<File>>,
    log: Arc<Mutex<File>>,
    hashmap: Arc<RwLock<HashMap<String, String>>>,
) -> () {
    let mut data_file = match data.write() {
        Ok(d) => d,
        Err(_) => {
            println!("ERROR: Lock de data en estado poisoned");
            return;
        }
    };
    // truncar data
    if data_file.set_len(0).is_err() {
        println!("ERROR: truncando data");
        return;
    }
    if data_file.seek(SeekFrom::Start(0)).is_err() {
        println!("ERROR: seek data");
        return;
    }

    let hashmap = match hashmap.read() {
        Ok(h) => h,
        Err(_) => {
            println!("Lock de hashmap en estado poissoned");
            return;
        }
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
    //trunco LOG
    let mut log_file = match log.lock() {
        Ok(l) => l,
        Err(_) => {
            println!("LOG en estado poissoned");
            return;
        }
    };
    if log_file.set_len(0).is_err() {
        println!("ERROR: truncando data");
        return;
    }
    if log_file.seek(SeekFrom::Start(0)).is_err() {
        println!("ERROR: seek data");
        return;
    }
}

pub fn manejar_delete(log:Arc<Mutex<File>>,hashmap: Arc<RwLock<HashMap<String, String>>>,clave: String) -> () {
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
                        return;
                    }
                }
}

pub fn manejar_set(
    log: Arc<Mutex<File>>,
    hashmap: Arc<RwLock<HashMap<String, String>>>,
    clave: String,
    valor: String,
) -> () {
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
            return;
        }
    }
}