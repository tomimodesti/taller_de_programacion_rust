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
    let mut lector = io::stdin().lock();
    let args: Vec<String> = std::env::args().collect();
    let direccion = match args.get(1) {
        Some(dir) => dir,
        None => {
            println!("ERROR: <{}>", KvErrores::MissingArgument.to_str());
            return;
        }
    };

    let timeout = match obtener_timeout() {
        Ok(t) => t,
        Err(_) => Duration::from_secs(10), // valor por defecto
    };

    let mut stream = match conectar_servidor(direccion, timeout) {
        Ok(s) => s,
        Err(e) => {
            println!("ERROR: <{}>", e.to_str());
            return;
        }
    };

    let reader_stream = match stream.try_clone() {
        Ok(s) => s,
        Err(_) => {
            println!("ERROR: <No se pudo clonar el stream>");
            return;
        }
    };
    let mut reader = BufReader::new(reader_stream);

    loop {
        print!("> ");
        match io::stdout().flush() {
            Ok(_) => {}
            Err(_) => {
                println!("Error: <Error de lectura input>");
                return;
            }
        }
        match obtener_entrada(&mut lector) {
            Ok(comando) => {
                //aca enviariamos al server y esperamos la respuesta
                //para esperar hasta que llegue algo o se acabe el timeout: set_read_timeout(Some(timeout)) en el socket y luego read() o recv()
                println!("Comando a enviar: <{}>", comando);
                //escribimos al server nuestro input y esperamos la respuesta
                match stream.write_all(format!("{}\n", comando).as_bytes()) {
                    Ok(_) =>
                    //si pudo escribir, esperamos la respuesta
                    {
                        let mut respuesta = String::new();
                        /*3 casos:
                        1) llega la respuesta y se imprime
                        2) error recuperable (ej: MISSING ARGUMENT), se imprime y continua : ERR_REC
                        3) error no recuperable (ej: TIMEOUT), se imprime y se cierra el cliente: ERR_NO_REC

                         */
                        let res = reader.read_line(&mut respuesta);
                        match res {
                            Ok(0) => {
                                println!("CONNECTION CLOSED");
                                return;
                            }
                            Ok(_) => match traducir_respuesta(respuesta.as_bytes()) {
                                ResultadoComunicacion::Continuar(r) => {
                                    println!("Respuesta del servidor: <{}>", r.trim())
                                }
                                ResultadoComunicacion::Cerrar(e) => {
                                    println!("ERROR <{}>", e.trim());
                                    println!("Cerrando cliente");
                                    return;
                                }
                            },
                            //error de timeout
                            Err(e) => {
                                match e.kind() {
                                    io::ErrorKind::TimedOut => {
                                        println!("ERROR: <TIMEOUT>");
                                    }
                                    io::ErrorKind::ConnectionReset => {
                                        println!("ERROR: <CONEXION PERDIDA>");
                                    }
                                    _ => {
                                        println!("ERROR: <{}>", e);
                                        println!("Cerrando cliente");
                                    }
                                }
                                return;
                            }
                        }
                    }
                    Err(e) => {
                        println!("ERROR: <{}>", e);
                    }
                };
            }
            Err(KvErrores::EOF) => break,
            Err(KvErrores::EMPTY) => continue,
            Err(KvErrores::Error(e)) => {
                println!("ERROR: <{}>", e);
                break;
            }
            Err(_) => {
                println!("Unknown Error");
                break;
            }
        }
    }
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

//Funcion que dado un buffer decide que tipo de error es el dado
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

/*
#[test]
fn test_obtener_entrada_exitosa() {
    let mut input = "set clave valor\n".as_bytes(); // Simula STDIN
    let resultado = obtener_entrada(&mut input);
    assert_eq!(resultado.unwrap(), "set clave valor");
}

#[test]
fn test_obtener_entrada_vacia() {
    let mut input = "\n".as_bytes();
    let resultado = obtener_entrada(&mut input);
    assert_eq!(resultado.unwrap_err(), "EMPTY");
}
    #[test]
fn test_con_dyn() {
    let mut mock_input = "get clave\n".as_bytes();
    // Se pasa como &mut dyn BufRead
    let res = obtener_entrada(&mut mock_input);
    assert_eq!(res.unwrap(), "get clave");
}
*/
