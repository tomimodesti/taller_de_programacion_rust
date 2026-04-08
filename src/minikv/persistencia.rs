//! archivo para metodos de persistencia
use crate::minikv::archivo::truncar_archivo;
use crate::minikv::errores::KvErrores;
use crate::minikv::estructuras::MensajePersistencia;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex, RwLock};

pub fn manejar_snapshot(
    data: Arc<RwLock<File>>,
    log: Arc<Mutex<File>>,
    hashmap: Arc<RwLock<HashMap<String, String>>>,
    sender: Sender<()>,
) {
    let resultado: Result<(), KvErrores> = (|| {
        let mut data_file = data
            .write()
            .map_err(|_| KvErrores::Error("LOCK DATA".into()))?;

        truncar_archivo(&mut data_file)?;

        let hashmap = hashmap
            .read()
            .map_err(|_| KvErrores::Error("LOCK HASHMAP".into()))?;

        for (clave, valor) in &*hashmap {
            let line = format!("\"{}\" \"{}\"\n", clave, valor.replace("\"", "\\\""));
            data_file
                .write_all(line.as_bytes())
                .map_err(|_| KvErrores::Error("WRITE DATA".into()))?;
        }

        drop(data_file);

        let mut log_file = log
            .lock()
            .map_err(|_| KvErrores::Error("LOCK LOG".into()))?;

        truncar_archivo(&mut log_file)?;

        Ok(())
    })();
    match sender.send(()) {
        Ok(_) => {
            if let Err(e) = resultado {
                println!("ERROR SNAPSHOT: {}", e.to_str());
            }
        }
        Err(_) => println!("No se pudo completar el snapshot"),
    }
}

pub fn manejar_delete(
    log: Arc<Mutex<File>>,
    hashmap: Arc<RwLock<HashMap<String, String>>>,
    clave: String,
    sender: Sender<()>,
) {
    let resultado: Result<(), KvErrores> = (|| {
        let log_line = format!("set \"{}\"\n", clave);

        let mut log_file = log
            .lock()
            .map_err(|_| KvErrores::Error("LOCK LOG".into()))?;

        log_file
            .write_all(log_line.as_bytes())
            .map_err(|e| KvErrores::Error(format!("WRITE LOG: {}", e)))?;

        let mut hashmap = hashmap
            .write()
            .map_err(|_| KvErrores::Error("LOCK HASHMAP".into()))?;

        hashmap.remove(&clave);

        Ok(())
    })();
    match sender.send(()) {
        Ok(_) => {
            if let Err(e) = resultado {
                println!("ERROR DELETE: {}", e.to_str());
            }
        }
        Err(_) => println!("No se pudo completar el snapshot"),
    }
}

pub fn manejar_set(
    log: Arc<Mutex<File>>,
    hashmap: Arc<RwLock<HashMap<String, String>>>,
    clave: String,
    valor: String,
    sender: Sender<()>,
) {
    let resultado: Result<(), KvErrores> = (|| {
        let log_line = format!("set \"{}\" \"{}\"\n", clave, valor.replace("\"", "\\\""));

        let mut log_file = log
            .lock()
            .map_err(|_| KvErrores::Error("LOCK LOG".into()))?;

        log_file
            .write_all(log_line.as_bytes())
            .map_err(|e| KvErrores::Error(format!("WRITE LOG: {}", e)))?;

        let mut hashmap = hashmap
            .write()
            .map_err(|_| KvErrores::Error("LOCK HASHMAP".into()))?;

        hashmap.insert(clave, valor);

        Ok(())
    })();
    match sender.send(()) {
        Ok(_) => {
            if let Err(e) = resultado {
                println!("ERROR SET: {}", e.to_str());
            }
        }
        Err(_) => println!("No se pudo completar el snapshot"),
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
            MensajePersistencia::Snapshot { tx } => {
                manejar_snapshot(data.clone(), log.clone(), hashmap.clone(), tx);
            }
            MensajePersistencia::Set { clave, valor, tx } => {
                manejar_set(log.clone(), hashmap.clone(), clave, valor, tx);
            }
            MensajePersistencia::Delete { clave, tx } => {
                manejar_delete(log.clone(), hashmap.clone(), clave, tx);
            }
        }
    }
}
