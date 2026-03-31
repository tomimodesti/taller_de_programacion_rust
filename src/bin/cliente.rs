//main del cliente
/*El cliente recibirá como argumento la dirección del servidor,
 y leerá las operaciones a enviar al servidor a través de STDIN.

cargo run --bin minikv-client -- 192.168.0.0:12345
 */

use minikv::minikv::comunicacion::ResultadoComunicacion;
use std::net::SocketAddr;
use std::{
    io::{self, BufRead, Read, Write},
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};
pub fn main() {
    let mut lector = io::stdin().lock();
    let args: Vec<String> = std::env::args().collect();
    let direccion = match args.get(1) {
        Some(dir) => dir,
        None => {
            println!("ERROR: <MISSING PUERTO>");
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
            println!("ERROR: <{}>", e);
            return;
        }
    };

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
                let _respuesta = match stream.write_all(format!("{}\n", comando).as_bytes()) {
                    Ok(_) =>
                    //si pudo escribir, esperamos la respuesta
                    {
                        let mut buffer = [0; 1024];
                        /*3 casos:
                        1) llega la respuesta y se imprime
                        2) error recuperable (ej: MISSING ARGUMENT), se imprime y continua : ERR_REC
                        3) error no recuperable (ej: TIMEOUT), se imprime y se cierra el cliente: ERR_NO_REC

                         */
                        let res = stream.read(&mut buffer);
                        match res {
                            Ok(n) => match traducir_respuesta(&buffer[..n]) {
                                ResultadoComunicacion::Continuar(r) => {
                                    println!("Respuesta del servidor: <{}>", r)
                                }
                                ResultadoComunicacion::Cerrar(e) => {
                                    println!("ERROR <{}>", e);
                                    println!("Cerrando cliente");
                                    return;
                                }
                            },
                            //error de timeout
                            Err(e) => {
                                match e.kind() {
                                    io::ErrorKind::TimedOut | io::ErrorKind::WouldBlock => {
                                        println!("ERROR: <TIMEOUT>");
                                        println!("Cerrando cliente");
                                    }
                                    io::ErrorKind::ConnectionAborted
                                    | io::ErrorKind::ConnectionReset => {
                                        println!("ERROR: <CONEXION PERDIDA>");
                                        println!("Cerrando cliente");
                                    }
                                    _ => {
                                        println!("ERROR: <ERROR DE COMUNICACION>");
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
            Err(e) if e == "EOF" => break,
            Err(e) if e == "EMPTY" => continue,
            Err(e) => {
                println!("ERROR: <{}>", e);
                break;
            }
        }
    }
}

fn obtener_entrada(lector: &mut dyn BufRead) -> Result<String, String> {
    let mut buffer = String::new();
    match lector.read_line(&mut buffer) {
        //Ctrl+D o comando vacio (EOF)
        Ok(0) => {
            println!("Cerrando cliente");
            Err("EOF".to_string())
        }
        Ok(_) => {
            let trim = buffer.trim().to_string();
            if trim.is_empty() {
                return Err("EMPTY".to_string());
            }
            Ok(trim)
        }
        Err(_) => Err("LECTURA FALLIDA".to_string()),
    }
}

fn conectar_servidor(direccion: &str, timeout: std::time::Duration) -> Result<TcpStream, String> {
    // conectarse al server
    //1) intentar conexion por TCP con la direccion dada
    let socket_addrs = match direccion.to_socket_addrs() {
        Ok(a) => a,
        Err(_) => {
            println!("ERROR: <DIRECCION INVALIDA>");
            return Err("DIRECCION INVALIDA".to_string());
        }
    };

    //2) conectamos el server y seteamos el timeout de lectura
    let lista_direcciones: Vec<SocketAddr> = socket_addrs.collect();
    let stream = match TcpStream::connect(&lista_direcciones[..]) {
        Ok(s) => s,
        Err(_) => {
            println!("ERROR: <CLIENT SOCKET BINDING>");
            return Err("CLIENT SOCKET BINDING".to_string());
        }
    };
    if let Err(_) = stream.set_read_timeout(Some(timeout)) {
        println!("ERROR: <NO SE PUDO SETEAR TIMEOUT>");
        return Err("NO SE PUDO SETEAR TIMEOUT".to_string());
    }
    Ok(stream)
}

fn obtener_timeout() -> Result<std::time::Duration, String> {
    match std::env::var("TIMEOUT") {
        Ok(t) => match t.parse::<u64>() {
            Ok(t) => Ok(std::time::Duration::from_secs(t)),
            Err(_) => {
                println!("ERROR: <TIMEOUT INVALIDO>");
                Err("TIMEOUT INVALIDO".to_string())
            }
        },
        Err(_) => {
            println!("ERROR: <MISSING TIMEOUT>");
            Err("MISSING TIMEOUT".to_string())
        }
    }
}

fn traducir_respuesta(buffer: &[u8]) -> ResultadoComunicacion {
    let respuesta = String::from_utf8_lossy(buffer).to_string();
    if respuesta.starts_with("ERR_REC") {
        ResultadoComunicacion::Continuar(respuesta)
    } else if respuesta.starts_with("ERR_NO_REC") {
        ResultadoComunicacion::Cerrar(respuesta)
    } else {
        ResultadoComunicacion::Continuar(respuesta)
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
