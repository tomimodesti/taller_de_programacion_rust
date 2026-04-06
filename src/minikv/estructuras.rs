//archivo para traits y estructuras comunes a todo el proyecto

use std::io::{Read, Seek, Write};
pub trait Storage: Read + Write + Seek + Send {}
impl<T: Read + Write + Seek + Send> Storage for T {}

pub enum MensajePersistencia {
    Snapshot,
    Set { clave: String, valor: String },
    Delete { clave: String },
}
