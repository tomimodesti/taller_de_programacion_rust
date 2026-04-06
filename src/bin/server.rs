use minikv::minikv::archivo::{abrir_archivos, cargar_hashmap};
use minikv::minikv::estructuras::MensajePersistencia;
use minikv::minikv::parseo::{parseo_comando, procesar_linea};
use std::collections::HashMap;
use std::fs::File;
use std::io::Seek;
use std::io::SeekFrom;
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
    let args = std::env::args().collect::<Vec<String>>();
    let direccion = match args.get(1) {
        Some(dir) => dir,
        None => {
            println!("ERROR: <INVALID ARGS>");
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
    println!("Servidor escuchando en <{}>", direccion);
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
    println!("Hashmap cargado con {} entradas", hashmap.len());
    let log_lock = Arc::new(Mutex::new(log_file));
    let data_lock = Arc::new(RwLock::new(data_file));
    let hashmap_lock = Arc::new(RwLock::new(hashmap));
    let (persistencia_sender, persistencia_receiver) = mpsc::channel::<MensajePersistencia>();
    println!("Estructuras de lock inicializadas");

    let log_clone = Arc::clone(&log_lock);
    let data_clone = Arc::clone(&data_lock);
    let hashmap_clone = Arc::clone(&hashmap_lock);

    //thread persistencia
    thread::spawn(move || {
        manejar_persistencia(persistencia_receiver, log_clone, data_clone, hashmap_clone);
    });
    println!("Thread de persistencia iniciado correctamente");
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
        //TODO: manejar cada mensaje de persistencia
        match mensaje {
            MensajePersistencia::Snapshot => {
                // manejar snapshot

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
            MensajePersistencia::Set { clave, valor } => {
                // manejar set
                // 1. pedir lock de escritura del log
                // 2. escribir en el log la operacion set (ej: set a 1)
                // 3. pedir lock de escritura del hashmap
                // 4. actualizar el hashmap con la nueva clave valor
                // 5. se liberan los locks al salir del scope de la funcion
                //    cuando busca un nuevo mensaje de persistencia
                let log_line: String =
                    format!("set \"{}\" \"{}\"\n", clave, valor.replace("\"", "\\\""));
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
            MensajePersistencia::Delete { clave } => {
                // manejar delete
                //1. pedir lock de esctritura del log
                //2. escribir en el log la operacion delete (ej: delete a)
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
        }
    }
}

fn inicializar_tcplistener(direccion: &str) -> Result<TcpListener, String> {
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

fn esperar_solicitudes(
    listener: TcpListener,
    persistencia_sender: Sender<MensajePersistencia>,
    hashmap_lock: Arc<RwLock<HashMap<String, String>>>,
) {
    println!("Esperando conexiones entrantes...");
    //este metodo bloquea el thread hasta que tenga algo que leer
    for stream in listener.incoming() {
        //stream es un result que puede ser Ok(TcpStream) o Err(e)
        match stream {
            Ok(s) => {
                println!("Nueva conexion entrante");
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
        // leemos de a una linea: ej: set a 1\n get a
        match reader.read_line(&mut linea) {
            Ok(0) => {
                println!("Conexion cerrada por el cliente");
                break; //muere el thread
            }
            Ok(_) => {
                let linea = linea.trim();
                println!("Mensaje recibido: <{}>", linea);
                //aca parseamos el mensaje y hacemos lo que corresponda (set, get, delete, snapshot)
                //luego de hacer la operacion, si es set o delete, enviamos un mensaje al thread de persistencia para que escriba en el log
                //si es snapshot, le decimos al thread de persistencia que escriba el snapshot completo en el data
                let args = procesar_linea(linea);
                let comando = match parseo_comando(args) {
                    Ok(c) => c,
                    Err(e) => {
                        let respuesta = format!("ERROR_REC: <{}>", e);
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
