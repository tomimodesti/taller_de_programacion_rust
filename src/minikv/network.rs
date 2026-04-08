//! Archivo para funciones relacionadas al apartado conexion
use crate::minikv::errores::KvErrores;
use std::net::TcpListener;

pub fn obtener_direccion() -> Result<String, KvErrores> {
    let args = std::env::args().collect::<Vec<String>>();
    match args.get(1) {
        Some(dir) => Ok(dir.clone()),
        None => Err(KvErrores::InvalidArgs),
    }
}

pub fn inicializar_tcplistener(direccion: String) -> Result<TcpListener, String> {
    let Ok(direccion) = direccion.parse::<std::net::SocketAddr>() else {
        return Err("PUERTO INVALIDO".into());
    };

    let Ok(stream) = TcpListener::bind(direccion) else {
        return Err("SERVIDOR SOCKET BINDING".into());
    };
    Ok(stream)
}
