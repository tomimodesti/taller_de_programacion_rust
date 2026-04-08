//! archivo para traits y estructuras comunes a todo el proyecto

use std::{
    io::{Read, Seek, Write},
    sync::mpsc::Sender,
};
pub trait Storage: Read + Write + Seek + Send {}
impl<T: Read + Write + Seek + Send> Storage for T {}

pub enum MensajePersistencia {
    Snapshot {
        tx: Sender<()>,
    },
    Set {
        clave: String,
        valor: String,
        tx: Sender<()>,
    },
    Delete {
        clave: String,
        tx: Sender<()>,
    },
}

pub enum LogCommand {
    SET { clave: String, valor: String },
    DELETE { clave: String },
}
