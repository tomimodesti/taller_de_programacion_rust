//main del cliente
/*El cliente recibirá como argumento la dirección del servidor,
 y leerá las operaciones a enviar al servidor a través de STDIN.

cargo run --bin minikv-client -- 192.168.0.0:12345
 */

use minikv::minikv::comunicacion::ResultadoComunicacion;
use minikv::minikv::errores::KvErrores;
use std::io::BufReader;
use std::net::SocketAddr;
use std::{
    io::{self, BufRead, Write},
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};
pub fn main() {
    let (mut stream, mut reader) = match inicializar_cliente() {
        Ok(c) => c,
        Err(e) => {
            println!("ERROR: <{}>", e.to_str());
            return;
        }
    };
    loop_cliente(&mut stream, &mut reader)
}

///Funcion que dado un buffer devuelve su contenido, esta preparado para identificar
/// errores del usuario para ingresar un comando
/// # ARGS
/// * lector: &mut dyn BufRead -> algo de lo que podemos leer (como STDIN o un buffer)
/// # Errores:
/// * errores de entrada de usuario
/// * errores de lectura fallida del buff
fn obtener_entrada(lector: &mut dyn BufRead) -> Result<String, KvErrores> {
    let mut buffer = String::new();
    match lector.read_line(&mut buffer) {
        //Ctrl+D o comando vacio (EOF)
        Ok(0) => Err(KvErrores::EOF),
        Ok(_) => {
            let trim = buffer.trim().to_string();
            if trim.is_empty() {
                return Err(KvErrores::EMPTY);
            }
            Ok(trim)
        }
        Err(_) => Err(KvErrores::Error("LECTURA FALLIDA".to_string())),
    }
}

///Funcion que dado la dirreccion a bindear y el timeout trata de acceder a la direccion
/// devolviendo un TcpStream si pudo conectarse a la direccion
/// # Args:
/// direccion: &str -> direccion a la que conectarse
/// timeout: std::time::Duration -> duracion maxima del timeout para esperar respuesta del servidor
/// # Errores:
/// * Error de puerto invalido -> si no hay puertos validos en la direccion indicada
/// * Error al tratar de acceder al stram
/// * Error al intentar setear el timeout de espera del servidor
fn conectar_servidor(
    direccion: &str,
    timeout: std::time::Duration,
) -> Result<TcpStream, KvErrores> {
    // conectarse al server
    //1) intentar conexion por TCP con la direccion dada
    let socket_addrs = match direccion.to_socket_addrs() {
        Ok(a) => a,
        Err(_) => {
            return Err(KvErrores::InvalidPuerto);
        }
    };
    //2) conectamos el server y seteamos el timeout de lectura
    let lista_direcciones: Vec<SocketAddr> = socket_addrs.collect();
    let stream = match TcpStream::connect(&lista_direcciones[..]) {
        Ok(s) => s,
        Err(_) => {
            return Err(KvErrores::ClientSocketBinding);
        }
    };
    if stream.set_read_timeout(Some(timeout)).is_err() {
        return Err(KvErrores::Error("NO SE PUDO SETEAR TIMEOUT".to_string()));
    }
    Ok(stream)
}

///Funcion que busca una variable de entorno llamada "TIMEOUT"
/// si la encuentra devuelve la duracion indicada en segundos
fn obtener_timeout() -> Result<std::time::Duration, KvErrores> {
    match std::env::var("TIMEOUT") {
        Ok(t) => match t.parse::<u64>() {
            Ok(t) => Ok(std::time::Duration::from_secs(t)),
            Err(_) => Err(KvErrores::Error("TIMEOUT INVALIDO".to_string())),
        },
        Err(_) => Err(KvErrores::Error("MISSING TIMEOUT".to_string())),
    }
}

///Funcion que dado un buffer decide que tipo de mensaje es el dado
/// si es un error recuperable, no recuperable o un mensaje de respuesta satisfactorio
/// (revisar en Comunicacion.rs)
fn traducir_respuesta(buffer: &[u8]) -> ResultadoComunicacion {
    let respuesta = String::from_utf8_lossy(buffer).to_string();

    if let Some(resultado) = respuesta.strip_prefix("ERR_REC:") {
        let limpio = resultado.trim();
        ResultadoComunicacion::Continuar(format!("ERROR: <{}>", limpio))
    } else if let Some(resultado) = respuesta.strip_prefix("ERR_NO_REC:") {
        let limpio = resultado.trim();
        ResultadoComunicacion::Cerrar(format!("ERROR: <{}>", limpio))
    } else {
        ResultadoComunicacion::Continuar(respuesta.trim().to_string())
    }
}

///Funcion que genera un TcpStream y un buff para mandar solicitudes y leer respuestas
/// del servidor donde estemos escuchando, eso lo marca la dirreccion pasada por STDIN
/// al inicio del programa
/// # Errores:
/// * MISSING ARGUMENT: si no fue indicada la dirreccion para mandar solicitudes
/// * Connection Closed: sino pudo conectarse al tcp o la conexion se cerro
fn inicializar_cliente() -> Result<(TcpStream, BufReader<TcpStream>), KvErrores> {
    let args: Vec<String> = std::env::args().collect();
    let direccion = match args.get(1) {
        Some(v) => Ok(v),
        None => Err(KvErrores::MissingArgument),
    }?;

    let timeout = match obtener_timeout() {
        Ok(s) => s,
        Err(_) => Duration::from_secs(10),
    };
    let stream = conectar_servidor(direccion, timeout)?;
    let reader_stream = stream
        .try_clone()
        .map_err(|_| KvErrores::ConnectionClosed)?;

    let reader = BufReader::new(reader_stream);
    Ok((stream, reader))
}

///Loop del cliente para enviar solicitudes al servidor y recibir, traducir y mostrar la respuesta del mismo
/// formas de salir del loop y terminar la conexion (EOF o un error de lectura de STDIN)
/// en caso de que se reciba un respuesta satisfacoria del servidor la conexion sigue abierta
/// esperando la siguiente query para el mismo, sin cerrar y abrir la conexion con el servidor
fn loop_cliente(stream: &mut TcpStream, reader: &mut BufReader<TcpStream>) {
    let mut lector = io::stdin().lock();
    loop {
        print!("> ");
        io::stdout().flush().ok();
        let comando = match obtener_entrada(&mut lector) {
            Ok(c) => c,
            Err(KvErrores::EOF) => break,
            Err(KvErrores::EMPTY) => continue,
            Err(e) => {
                println!("ERROR: <{}>", e.to_str());
                break;
            }
        };
        println!("Comando a enviar: <{}>", comando);
        if let Err(e) = enviar_comando(stream, &comando) {
            println!("ERROR: <{}>", e.to_str());
            continue;
        }
        match recibir_respuesta(reader) {
            ResultadoComunicacion::Continuar(r) => {
                println!("Respuesta del servidor: <{}>", r.trim())
            }
            ResultadoComunicacion::Cerrar(e) => {
                println!("ERROR <{}>", e.trim());
                break;
            }
        }
    }
}

///Funcion que permite enviar un comando a traves del tcpstream al servidor que este escuchando en esa dirreccion
/// el formato de envio es la linea indicada como argumento con un salto de linea, por lo que
/// no es necesario agregarla por parte del usuario, se hace automaticamente
fn enviar_comando(stream: &mut TcpStream, comando: &str) -> Result<(), KvErrores> {
    stream
        .write_all(format!("{}\n", comando).as_bytes())
        .map_err(|_| KvErrores::ConnectionClosed)
}

///Funcion que espera la respuesta del servidor, puede recibir multiples respuestas
/// satisfactoria, un error no recuperable o terminar ya que se consumio el timeout de escucha
fn recibir_respuesta(reader: &mut BufReader<TcpStream>) -> ResultadoComunicacion {
    let mut respuesta = String::new();
    match reader.read_line(&mut respuesta) {
        Ok(0) => ResultadoComunicacion::Cerrar("CONNECTION CLOSED".to_string()),
        Ok(_) => traducir_respuesta(respuesta.as_bytes()),
        Err(e) => match e.kind() {
            io::ErrorKind::TimedOut => ResultadoComunicacion::Cerrar("TIMEOUT".to_string()),
            io::ErrorKind::ConnectionReset => {
                ResultadoComunicacion::Cerrar("CONEXION PERDIDA".to_string())
            }
            _ => ResultadoComunicacion::Cerrar("ERROR DE COMUNICACION".to_string()),
        },
    }
}

#[test]
fn test_obtener_entrada_exitosa() {
    let mut input = "set clave valor\n".as_bytes(); // Simula STDIN
    let resultado = obtener_entrada(&mut input);
    assert_eq!(resultado.unwrap(), "set clave valor");
}

#[test]
fn test_con_dyn() {
    let mut mock_input = "get clave\n".as_bytes();
    // Se pasa como &mut dyn BufRead
    let res = obtener_entrada(&mut mock_input);
    assert_eq!(res.unwrap(), "get clave");
}

#[test]
fn entrada_valida() {
    use std::io::Cursor;
    let input = b"SET clave valor\n";
    let mut cursor = Cursor::new(input);

    let res = obtener_entrada(&mut cursor).unwrap();
    assert_eq!(res, "SET clave valor");
}

#[test]
fn entrada_vacia() {
    use std::io::Cursor;
    let input = b"\n";
    let mut cursor = Cursor::new(input);

    let res = obtener_entrada(&mut cursor);
    assert!(matches!(res, Err(KvErrores::EMPTY)));
}

#[test]
fn entrada_eof() {
    use std::io::Cursor;
    let input = b"";
    let mut cursor = Cursor::new(input);

    let res = obtener_entrada(&mut cursor);
    assert!(matches!(res, Err(KvErrores::EOF)));
}

#[test]
fn respuesta_error_recuperable() {
    let input = b"ERR_REC: clave invalida";

    match traducir_respuesta(input) {
        ResultadoComunicacion::Continuar(msg) => {
            assert_eq!(msg, "ERROR: <clave invalida>");
        }
        _ => panic!("Esperaba Continuar"),
    }
}

#[test]
fn respuesta_error_no_recuperable() {
    let input = b"ERR_NO_REC: fatal";

    match traducir_respuesta(input) {
        ResultadoComunicacion::Cerrar(msg) => {
            assert_eq!(msg, "ERROR: <fatal>");
        }
        _ => panic!("Esperaba Cerrar"),
    }
}

#[test]
fn respuesta_normal() {
    let input = b"OK";

    match traducir_respuesta(input) {
        ResultadoComunicacion::Continuar(msg) => {
            assert_eq!(msg, "OK");
        }
        _ => panic!("Esperaba Continuar"),
    }
}

#[test]
fn conectar_servidor_ok() {
    use std::net::TcpListener;
    use std::thread;
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    thread::spawn(move || {
        let _ = listener.accept();
    });

    let res = conectar_servidor(&addr.to_string(), std::time::Duration::from_secs(1));
    assert!(res.is_ok());
}
