//! Archivo para funciones relacionadas al apartado conexion 
use  crate::minikv::errores::KvErrores;
use std::net::TcpListener;

pub fn obtener_direccion() -> Result<String, KvErrores> {
    let args = std::env::args().collect::<Vec<String>>();
    match args.get(1) {
        Some(dir) => Ok(dir.clone()),
        None => return Err(KvErrores::InvalidArgs),
    }
}


pub fn inicializar_tcplistener(direccion: String) -> Result<TcpListener, String> {
    let direccion = match direccion.parse::<std::net::SocketAddr>() {
        Ok(d) => d,
        Err(_) => {
            return Err("PUERTO INVALIDO".into());
        }
    };
    let stream = match TcpListener::bind(direccion) {
        Ok(s) => s,
        Err(_) => {
            return Err("SERVIDOR SOCKET BINDING".into());
        }
    };
    Ok(stream)
}

