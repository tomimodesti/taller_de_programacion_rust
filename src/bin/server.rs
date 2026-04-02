//main del server

/*
El servidor recibirá como argumento la dirección a través
 de la cual escuchará conexiones entrantes de los operadores.

cargo run --bin minikv-server -- 192.168.0.0:12345
 */
use std::{net::TcpListener, sync::mpsc, thread};
use std::collections::HashMap;
use std::io::{self, Write};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Arc,RwLock};
use std::io::{Read,Seek};
const DATA_PATH: &str = ".minikv.data";
const LOG_PATH:&str = ".minikv.log";
pub trait Storage : Read + Write + Seek + Send {}
impl <T: Read + Write + Seek + Send> Storage for T {}
pub fn main() {
    //paso 1: abrirse y esperar las solicitudes
    let args = std::env::args().collect::<Vec<String>>();
    let direccion = match args.get(1) {
        Some(dir) => dir,
        None => {
            println!("ERROR: <INVALID ARGS>");
            return;
        }
    };
    let direccion = match direccion.parse::<std::net::SocketAddr>() {
        Ok(d) => d,
        Err(_) => {
            println!("ERROR: <PUERTO INVALIDO>");
            return;
        }
    };
    let mut stream = match TcpListener::bind(direccion) {
        Ok(s) => s,
        Err(_) => {
            println!("ERROR: <SERVIDOR SOCKET BINDING>");
            return;
        }
    };// aca ya tendriamos el socket preparado para escuchar las conexiones entrantes
    println!("Servidor escuchando en <{}>", direccion);
    //abrir locks de escritura lectura para LOG y DATA
    let data_file = match abrir_archivo(DATA_PATH,false) {
        Ok(f) => f,
        Err(e) => {
            println!("{}",e);
            return;
        }
    };
    let log_file = match abrir_archivo(LOG_PATH,true) {
        Ok(f) => f,
        Err(e) => {
            println!("{}",e);
            return;
        }
    };
    //TODO: cargar el hashmap en memoria
    let mut hashmap = HashMap::new();
    hashmap = cargar_hashmap_

    let log_lock = Arc::new(RwLock::new(log_file));
    let data_lock = Arc::new(RwLock::new(data_file));
    //preparamos para el archivo log un thread especial que escribe en el, 
    // al cual se comunican todos los threads para que solo uno maneje el archivo y el lock del mismo
    // para esto tendria un channel entre todos los threads y el thread del log, 
    //y cada vez que un thread quiera escribir en el log, 
    // le manda un mensaje a ese thread con la info a escribir
    //el channel toma el valor de lo primero que se envie por el, ej: String
    let (log_sender,log_receiver) = mpsc::channel();
    //este channel funcionaria para que un solo thread escriba en el data y log, sera el encargado de realizar los snapshot
    //asi los demas threads solo enviarian un mensaje de "SNAPSHOT" o similar al thread asignado
    let (data_sender,data_receiver) = mpsc::channel(); //
    let data_lock_clone = 
    let log_thread = thread::spawn(move || {
        //aqui se maneja el archivo log,
        // se queda esperando mensajes de los otros threads para escribir
        // en el log
        // el move indica que toma el ownership
        // de todas las variables que se usen dentro del thread.
        manejar_log(log_receiver,LOG_PATH);
    });
    let snapshot_thread = thread::spawn(move ||{
    //thread que solo realiza los snaposhot, espera un mensaje de los threads
    // asi solo 1 thread maneja el archivo data,
    // y se evitan problemas de concurrencia,
        manejar_snapshot();
    });
    //paso 2: al obtener una solicitud, crear un thread que la maneje
    //paso 3: seguir esperando
}

fn manejar_log(log_receiver: Receiver<String>,log_path: Box<dyn Write>) {
    //espera a que haya algo para escribir en el log
    for mensaje in log_receiver {
        let linea = format!("{}\n",mensaje);

    }
}

fn abrir_archivo(path: &str,append: bool) -> io::Result<Box<dyn Storage>> {
    //abrir el archivo, si no existe lo crea
    //si append es true, se abre para agregar al final, sino se sobreescribe
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(append)
        .open(path);
    match file {
        Ok(f) => Ok(Box::new(f)),
        Err(_) => Err("ERROR: <ARCHIVO INACCESIBLE>".into()),
    }
    
}

fn manejar_snapshot() {
    //espera a que haya un mensaje para realizar un snapshot
    //cuando lo recibe, bloquea el acceso al archivo data, 
    // realiza el snapshot y luego desbloquea el acceso
}
