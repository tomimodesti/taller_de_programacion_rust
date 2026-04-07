use minikv::minikv::archivo::{abrir_archivos, cargar_hashmap};
use minikv::minikv::network::{obtener_direccion,inicializar_tcplistener};
use minikv::minikv::persistencia::{manejar_delete,manejar_set,manejar_snapshot};
use minikv::minikv::estructuras::MensajePersistencia;
use minikv::minikv::parseo::{parseo_comando, procesar_linea};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex, RwLock};
use std::{net::TcpListener, sync::mpsc, thread};
//main del server
/*
El servidor recibirá como argumento la dirección a través
 de la cual escuchará conexiones entrantes de los operadores.

cargo run --bin minikv-server -- 192.168.0.0:12345
 */

const DATA_PATH: &str = ".minikv.data";
const LOG_PATH: &str = ".minikv.log";

pub fn main() {
    let direccion = match obtener_direccion() {
        Ok(d) => d,
        Err(e) => {
            println!("{}", e.to_str());
            return;
        }
    };
    let listener: TcpListener = match inicializar_tcplistener(direccion) {
        Ok(l) => l,
        Err(e) => {
            println!("ERROR: <{}>", e);
            return;
        }
    };
    let (mut data_file, mut log_file) = match abrir_archivos(DATA_PATH, LOG_PATH) {
        Ok((a, b)) => (a, b),
        Err(e) => {
            println!("ERROR: <{}>", e.to_str());
            return;
        }
    };
    let hashmap: HashMap<String, String> = match cargar_hashmap(&mut data_file, &mut log_file) {
        Ok(h) => h,
        Err(e) => {
            println!("ERROR: <{}>", e.to_str());
            return;
        }
    };
    inicializar_threads(data_file, log_file, hashmap, listener);
}

fn inicializar_threads(
    data_file: File,
    log_file: File,
    hashmap: HashMap<String, String>,
    listener: TcpListener,
) {
    let log_lock = Arc::new(Mutex::new(log_file));
    let data_lock = Arc::new(RwLock::new(data_file));
    let hashmap_lock = Arc::new(RwLock::new(hashmap));
    let (persistencia_sender, persistencia_receiver) = mpsc::channel::<MensajePersistencia>();
    let log_clone = Arc::clone(&log_lock);
    let data_clone = Arc::clone(&data_lock);
    let hashmap_clone = Arc::clone(&hashmap_lock);
    //thread persistencia
    thread::spawn(move || {
        manejar_persistencia(persistencia_receiver, log_clone, data_clone, hashmap_clone);
    });
    let hashmap_clone = Arc::clone(&hashmap_lock);
    esperar_solicitudes(listener, persistencia_sender, hashmap_clone);
}

fn manejar_persistencia(
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

fn esperar_solicitudes(
    listener: TcpListener,
    persistencia_sender: Sender<MensajePersistencia>,
    hashmap_lock: Arc<RwLock<HashMap<String, String>>>,
) {
    //este metodo bloquea el thread hasta que tenga algo que leer
    for stream in listener.incoming() {
        //stream es un result que puede ser Ok(TcpStream) o Err(e)
        match stream {
            Ok(s) => {
                let persistencia_sender_clone = persistencia_sender.clone();
                let hashmap_clone = Arc::clone(&hashmap_lock);
                //thread por cliente
                thread::spawn(move || {
                    manejar_solicitud(s, persistencia_sender_clone, hashmap_clone);
                });
            }
            Err(_) => {
                println!("ERROR: <No se pudo aceptar conexion entrante>")
            }
        }
    }
}


fn manejar_solicitud(
    mut stream: TcpStream,
    persistencia_sender: Sender<MensajePersistencia>,
    hashmap_lock: Arc<RwLock<HashMap<String, String>>>,
) {
    //manejamos las solicitudes del cliente
    let reader_stream = match stream.try_clone() {
        Ok(s) => s,
        Err(_) => {
            println!("ERROR: <No pudo leer la solicitud del cliente>");
            return;
        }
    };
    let mut reader = BufReader::new(reader_stream);
    let mut linea = String::new();
    loop {
        linea.clear();
        match reader.read_line(&mut linea) {
            Ok(0) => {
                println!("Conexion cerrada por el cliente");
                break; //muere el thread
            }
            Ok(_) => {
                let linea = linea.trim();
                let args = procesar_linea(linea);
                let comando = match parseo_comando(args) {
                    Ok(c) => c,
                    Err(e) => {
                        let respuesta = format!("ERROR_REC: <{}>", e.to_str());
                        if escribir_respuesta(&mut stream, respuesta).is_err() {
                            break;
                        }
                        continue; //espera siguiente solicitud
                    }
                };
                //ejecutamos el comando y obtenemos la respuesta
                let hashmap_lock_clone = Arc::clone(&hashmap_lock);
                let persistencia_sender_clone = persistencia_sender.clone();
                match comando.ejecutar(hashmap_lock_clone, persistencia_sender_clone) {
                    Ok(respuesta) => {
                        if escribir_respuesta(&mut stream, respuesta).is_err() {
                            break;
                        }
                        //espera siguiente solicitud
                    }
                    Err(e) => {
                        let respuesta = format!("ERROR: <{}>", e);
                        if escribir_respuesta(&mut stream, respuesta).is_err() {
                            break;
                        }
                        continue; //espera siguiente solicitud
                    }
                }
            }
            Err(_) => {
                println!("ERROR: <Error de lectura del cliente>");
                break; //muere el thread
            }
        }
    }
}

fn escribir_respuesta(stream: &mut TcpStream, respuesta: String) -> Result<(), String> {
    match writeln!(stream, "{}", respuesta) {
        Ok(_) => match stream.flush() {
            Ok(_) => Ok(()),
            Err(_) => {
                println!("ERROR: <No pudo enviar respuesta al cliente>");
                Err("ERROR DE ESCRITURA".to_string())
            }
        },
        Err(_) => {
            println!("ERROR: <No pudo enviar respuesta al cliente>");
            Err("ERROR DE ESCRITURA".to_string())
        }
    }
}


#[test]
fn listener_valido() {
    let res = inicializar_tcplistener("127.0.0.1:0".to_string());
    assert!(res.is_ok());
}

#[test]
fn listener_invalido() {
    let res = inicializar_tcplistener("direccion_invalida".to_string());
    assert!(res.is_err());
}

#[test]
fn escribir_respuesta_ok() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let handle = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        escribir_respuesta(&mut stream, "OK".to_string()).unwrap();
    });

    let stream = TcpStream::connect(addr).unwrap();
    let mut reader = BufReader::new(stream);

    let mut linea = String::new();
    reader.read_line(&mut linea).unwrap();

    assert_eq!(linea.trim(), "OK");

    handle.join().unwrap();
}

#[test]
fn test_set_y_get() {
    use std::collections::HashMap;
    use std::sync::mpsc;
    use std::sync::{Arc, RwLock};

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let hashmap = Arc::new(RwLock::new(HashMap::new()));
    let (tx, _rx) = mpsc::channel();

    // server thread
    let hashmap_clone = Arc::clone(&hashmap);
    let tx_clone = tx.clone();

    thread::spawn(move || {
        let (stream, _) = listener.accept().unwrap();
        manejar_solicitud(stream, tx_clone, hashmap_clone);
    });

    // cliente
    let mut stream = TcpStream::connect(addr).unwrap();

    writeln!(stream, "set clave valor\n").unwrap();
    stream.flush().unwrap();

    let mut reader = BufReader::new(stream);
    let mut response = String::new();

    reader.read_line(&mut response).unwrap();
    println!("{}", response);
    assert!(response.contains("OK") || response.contains("clave"));
}
