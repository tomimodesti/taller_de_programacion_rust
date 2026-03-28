//main del cliente
/*El cliente recibirá como argumento la dirección del servidor,
 y leerá las operaciones a enviar al servidor a través de STDIN.

cargo run --bin minikv-client -- 192.168.0.0:12345
 */

use std::io::{self, Write};

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    let _direccion = match args.get(1) {
        Some(dir) => dir,
        None => {
            println!("ERROR: <MISSING PUERTO>");
            return;
        }
    };
    // conectarse al server

    loop {
        print!("> ");
        match  io::stdout().flush() {
            Ok(_) => {} ,
            Err(_) => {println!("Error: <Error de lectura input>");
            return;}
            
        }
        match obtener_entrada() {
            Ok(comando) =>
            //parseamos
            {
                let _partes :Vec<String> = comando.split(' ').map(|s| s.to_string()).collect();
                println!("{}",comando);

            }
            Err(e) if e == "EOF" => break,
            Err(e) if e == "EMPTY" => continue,
            Err(e) => {
                println!("ERROR: {}", e);
                break;
            }
        }
    }
}

fn obtener_entrada() -> Result<String, String> {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
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
