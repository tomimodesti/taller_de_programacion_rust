
use std::fs::File;
use std::io::Read;
use std::net::TcpStream;
use minikv::minikv::archivo::{abrir_archivo, cargar_hashmap};
use std::io::{Seek, Write};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex, RwLock};
use std::{net::TcpListener, sync::mpsc, thread};
use std::collections::HashMap;
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
    let (persistencia_sender, persistencia_receiver) = mpsc::channel();

    println!("Estructuras de lock inicializadas");
    //aca tenemos los locks de las estructuras que vamos a compartir

    let log_clone = Arc::clone(&log_lock);
    let data_clone = Arc::clone(&data_lock);
    let hashmap_clone = Arc::clone(&hashmap_lock);

    let persistencia_thread = thread::spawn(move || {
        //se maneja la persistencia en un thread aparte,
        //asi el thread principal del server solo se encarga de aceptar conexiones y
        // delegar el manejo de cada solicitud a un thread aparte,
        //sin preocuparse por la persistencia
        manejar_persistencia(persistencia_receiver, log_clone, data_clone, hashmap_clone);
    });

    println!("Thread de persistencia iniciado correctamente" );

    //paso 3: esperar solicitudes entrantes y delegar su manejo a threads aparte
    let hashmap_clone = Arc::clone(&hashmap_lock);
    esperar_solicitudes(listener, persistencia_sender, hashmap_clone);
}

fn manejar_persistencia<W: Write + Seek + Send + 'static>(
    persistencia_receiver: Receiver<String>,
    log: Arc<Mutex<W>>,
    data: Arc<RwLock<W>>,
    hashmap: Arc<RwLock<HashMap<String, String>>>,
) {
    //espera a que haya algo para escribir (set, delete o snapshot)
    for mensaje in persistencia_receiver {
        let linea = format!("{}\n", mensaje);
        println!("{}", linea);
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
    persistencia_sender: Sender<String>,
    hashmap_lock: Arc<RwLock<HashMap<String, String>>>,
) {
    println!("Esperando conexiones entrantes...");
    //este metodo bloquea el thread hasta que tenga algo que leer
    for stream in listener.incoming() {
        //stream es un result que puede ser Ok(TcpStream) o Err(e)
        match stream {
            Ok(s) =>{
                println!("Nueva conexion entrante");
                let persistencia_sender_clone = persistencia_sender.clone();
                let hashmap_clone = Arc::clone(&hashmap_lock);
                let thread_usuario = thread::spawn(move || {
                    manejar_solicitud(s,persistencia_sender_clone, hashmap_clone);
                }
                );
            },
            Err(_) => {
                println!("ERROR: <No se pudo aceptar conexion entrante>")
            },
        }
    }
}

fn manejar_solicitud(
    stream: TcpStream,
    persistencia_sender: Sender<String>,
    hashmap_lock: Arc<RwLock<HashMap<String, String>>>
) {
    //manejamos las solicitudes del cliente
    let mut buffer = [0;1024];
    for solicitud in stream.read(&mut buffer) {
        match solicitud {
            Ok(bytes_leidos) => {
                let mensaje = String::from_utf8_lossy(&buffer[..bytes_leidos]);
                println!("Solicitud recibida: {}", mensaje);
                //aca parseamos el mensaje y hacemos lo que corresponda
                //si es un set, update o delete, le mandamos un mensaje al thread de persistencia
                //para que se encargue de escribirlo en el log y actualizar el data si es necesario
            },
            Err(_) => {
                println!("ERROR: <No se pudo leer la solicitud del cliente>");
                break;
            }
        }

    }
}

