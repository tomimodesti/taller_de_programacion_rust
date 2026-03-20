
const DATA_PATH: &str = ".minikv.data";
const LOG_PATH: &str = ".minikv.log";


///Funcion que crea un archivo, si ya existe lo sobreescribe
/// # Arguments
/// * `path` - Ruta del archivo a crear - &str
pub fn crear_archivo(path: &str) -> Result<File, Error> {
    match std::fs::File::create(path) {
        Ok(file) => Ok(file),
        Err(e) => Err(format!("Error al crear el archivo {}: {}", path, e)),
    }
}


///Funcion que busca un archivo por un path dado
/// devuelve un Path al archivo si existe sino devuelve un error (no lo maneja)
pub fn buscar_archivo(path: &str) -> Result<File, String> {
    let f = File::open(path); //devuelve un error o un file
    match f {
        //si lo encuentra devuelve el file, sino devuelve un error (no me interesa devolver un mensaje)
        Ok(file) => Ok(file),
        Err(e) => Err(format!("Error al buscar el archivo {}: {}", path, e)),
    }
}

///Funcion que abre un archivo para appendear, si no existe lo crea
/// # Arguments
/// * `path` - Ruta del archivo a abrir - &str
/// devuelve un File abierto para appendear o un mensaje de error si no se pudo abrir ni crear el archivo
/// devuelve el File o un mensaje de error
pub fn abrir_para_appendear(path: &str) -> Result<File,String> {
    match OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(path) {
        Ok(file) => Ok(file),
        Err(_) => {
            match crear_archivo(path) {
                Ok(file) => Ok(file),
                Err(e) => Err(format!("Error al crear el archivo {}: {}", path, e)),
            }
        }
    }                      
}
