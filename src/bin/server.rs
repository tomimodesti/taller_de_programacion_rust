use minikv::minikv::archivo::{abrir_archivo, cargar_hashmap};
use minikv::minikv::estructuras::MensajePersistencia;
use minikv::minikv::parseo::parseo_comando;
use std::collections::HashMap;
use std::fs::File;
use std::io::Seek;
use std::io::{BufRead, BufReader, Read, Write};
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
    //paso 1: inicializar el servidor, cargar el data, log y hashmap
    //paso 2: iniciar thread de persistencia
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
    // aca ya tendriamos el socket preparado para escuchar las conexiones entrantes
    println!("Servidor escuchando en <{}>", direccion);

    let mut data_file: File = match abrir_archivo(DATA_PATH, false) {
        Ok(f) => f,
        Err(e) => {
            println!("ERROR: <{}>", e);
            return;
        }
    };
    let mut log_file: File = match abrir_archivo(LOG_PATH, true) {
        Ok(f) => f,
        Err(e) => {
            println!("ERROR: <{}>", e);
            return;
        }
    };

    let mut hashmap: HashMap<String, String> = match cargar_hashmap(&mut data_file, &mut log_file) {
        Ok(h) => h,
        Err(e) => {
            println!("ERROR: <{}>", e);
            return;
        }
    };
    println!("Hashmap cargado con {} entradas", hashmap.len());
    //aca ya tendriamos el data, log y el hashmap preparados

    let log_lock = Arc::new(Mutex::new(log_file));
    let data_lock = Arc::new(RwLock::new(data_file));
    let hashmap_lock = Arc::new(RwLock::new(hashmap));
    let (persistencia_sender, persistencia_receiver) = mpsc::channel::<MensajePersistencia>();
    println!("Estructuras de lock inicializadas");
    //aca tenemos los locks de las estructuras que vamos a compartir

    let log_clone = Arc::clone(&log_lock);
    let data_clone = Arc::clone(&data_lock);
    let hashmap_clone = Arc::clone(&hashmap_lock);

    //thread persistencia
    thread::spawn(move || {
        //se maneja la persistencia en un thread aparte,
        //asi el thread principal del server solo se encarga de aceptar conexiones y
        // delegar el manejo de cada solicitud a un thread aparte,
        //sin preocuparse por la persistencia
        manejar_persistencia(persistencia_receiver, log_clone, data_clone, hashmap_clone);
    });

    println!("Thread de persistencia iniciado correctamente");

    //paso 3: esperar solicitudes entrantes y delegar su manejo a threads aparte
    let hashmap_clone = Arc::clone(&hashmap_lock);
    esperar_solicitudes(listener, persistencia_sender, hashmap_clone);
}

fn manejar_persistencia<W: Write + Seek + Send + 'static>(
    persistencia_receiver: Receiver<MensajePersistencia>,
    log: Arc<Mutex<W>>,
    data: Arc<RwLock<W>>,
    hashmap: Arc<RwLock<HashMap<String, String>>>,
) {
    //espera a que haya algo para escribir (set, delete o snapshot)
    for mensaje in persistencia_receiver {
        //TODO: manejar cada mensaje de persistencia
        match mensaje {
            MensajePersistencia::Snapshot => {
                // manejar snapshot
            }
            MensajePersistencia::Set { clave, valor } => {
                // manejar set
            }
            MensajePersistencia::Delete { clave } => {
                // manejar delete
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
                println!("Mensaje recibido: <{}>", linea);
                //aca parseamos el mensaje y hacemos lo que corresponda (set, get, delete, snapshot)
                //luego de hacer la operacion, si es set o delete, enviamos un mensaje al thread de persistencia para que escriba en el log
                //si es snapshot, le decimos al thread de persistencia que escriba el snapshot completo en el data
                let args = linea
                    .trim()
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                let comando = match parseo_comando(args) {
                    Ok(c) => c,
                    Err(e) => {
                        let respuesta = format!("ERROR: <{}>", e);
                        if escribir_respuesta(&mut stream, respuesta).is_err() {
                            //ejemplo de respuesta: ERROR: <MISSING ARGUMENT> o ERROR: <UNKNOWN COMMAND>
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
